// Copyright 2018 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::io::{self};
use std::mem::size_of;
use std::sync::Arc;

use arch::{get_serial_cmdline, GetSerialCmdlineError, RunnableLinuxVm, VmComponents, VmImage};
use base::{Event, MemoryMappingBuilder};
use devices::serial_device::{SerialHardware, SerialParameters};
use devices::{
    Bus, BusDeviceObj, BusError, IrqChip, IrqChipAArch64, PciAddress, PciConfigMmio, PciDevice,
    ProtectionType,
};
use hypervisor::{DeviceKind, Hypervisor, HypervisorCap, VcpuAArch64, VcpuFeature, VmAArch64};
use minijail::Minijail;
use remain::sorted;
use resources::SystemAllocator;
use sync::Mutex;
use thiserror::Error;
use vm_control::BatteryType;
use vm_memory::{GuestAddress, GuestMemory, GuestMemoryError};

mod fdt;

// We place the kernel at offset 8MB
const AARCH64_KERNEL_OFFSET: u64 = 0x80000;
const AARCH64_FDT_MAX_SIZE: u64 = 0x200000;
const AARCH64_INITRD_ALIGN: u64 = 0x1000000;

// These constants indicate the address space used by the ARM vGIC.
const AARCH64_GIC_DIST_SIZE: u64 = 0x10000;
const AARCH64_GIC_CPUI_SIZE: u64 = 0x20000;

// This indicates the start of DRAM inside the physical address space.
const AARCH64_PHYS_MEM_START: u64 = 0x80000000;
const AARCH64_AXI_BASE: u64 = 0x40000000;
const AARCH64_PLATFORM_MMIO_SIZE: u64 = 0x800000;

// FDT is placed at the front of RAM when booting in BIOS mode.
const AARCH64_FDT_OFFSET_IN_BIOS_MODE: u64 = 0x0;
// Therefore, the BIOS is placed after the FDT in memory.
const AARCH64_BIOS_OFFSET: u64 = AARCH64_FDT_MAX_SIZE;
const AARCH64_BIOS_MAX_LEN: u64 = 1 << 20;

const AARCH64_PROTECTED_VM_FW_MAX_SIZE: u64 = 0x200000;
const AARCH64_PROTECTED_VM_FW_START: u64 =
    AARCH64_PHYS_MEM_START - AARCH64_PROTECTED_VM_FW_MAX_SIZE;

const AARCH64_PVTIME_IPA_MAX_SIZE: u64 = 0x10000;
const AARCH64_PVTIME_IPA_START: u64 = AARCH64_PROTECTED_VM_FW_START - AARCH64_PVTIME_IPA_MAX_SIZE;
const AARCH64_PVTIME_SIZE: u64 = 64;

// These constants indicate the placement of the GIC registers in the physical
// address space.
const AARCH64_GIC_DIST_BASE: u64 = AARCH64_AXI_BASE - AARCH64_GIC_DIST_SIZE;
const AARCH64_GIC_CPUI_BASE: u64 = AARCH64_GIC_DIST_BASE - AARCH64_GIC_CPUI_SIZE;
const AARCH64_GIC_REDIST_SIZE: u64 = 0x20000;

// PSR (Processor State Register) bits
const PSR_MODE_EL1H: u64 = 0x00000005;
const PSR_F_BIT: u64 = 0x00000040;
const PSR_I_BIT: u64 = 0x00000080;
const PSR_A_BIT: u64 = 0x00000100;
const PSR_D_BIT: u64 = 0x00000200;

macro_rules! offset__of {
    ($str:ty, $($field:ident).+ $([$idx:expr])*) => {
        unsafe { &(*(0 as *const $str))$(.$field)*  $([$idx])* as *const _ as usize }
    }
}

const KVM_REG_ARM64: u64 = 0x6000000000000000;
const KVM_REG_SIZE_U64: u64 = 0x0030000000000000;
const KVM_REG_ARM_COPROC_SHIFT: u64 = 16;
const KVM_REG_ARM_CORE: u64 = 0x0010 << KVM_REG_ARM_COPROC_SHIFT;

/// Gives the ID for a register to be used with `set_one_reg`.
///
/// Pass the name of a field in `user_pt_regs` to get the corresponding register
/// ID, e.g. `arm64_core_reg!(pstate)`
///
/// To get ID for registers `x0`-`x31`, refer to the `regs` field along with the
/// register number, e.g. `arm64_core_reg!(regs, 5)` for `x5`. This is different
/// to work around `offset__of!(kvm_sys::user_pt_regs, regs[$x])` not working.
macro_rules! arm64_core_reg {
    ($reg: tt) => {
        KVM_REG_ARM64
            | KVM_REG_SIZE_U64
            | KVM_REG_ARM_CORE
            | ((offset__of!(kvm_sys::user_pt_regs, $reg) / 4) as u64)
    };
    (regs, $x: literal) => {
        KVM_REG_ARM64
            | KVM_REG_SIZE_U64
            | KVM_REG_ARM_CORE
            | (((offset__of!(kvm_sys::user_pt_regs, regs) + ($x * size_of::<u64>())) / 4) as u64)
    };
}

fn get_kernel_addr() -> GuestAddress {
    GuestAddress(AARCH64_PHYS_MEM_START + AARCH64_KERNEL_OFFSET)
}

fn get_bios_addr() -> GuestAddress {
    GuestAddress(AARCH64_PHYS_MEM_START + AARCH64_BIOS_OFFSET)
}

// Serial device requires 8 bytes of registers;
const AARCH64_SERIAL_SIZE: u64 = 0x8;
// This was the speed kvmtool used, not sure if it matters.
const AARCH64_SERIAL_SPEED: u32 = 1843200;
// The serial device gets the first interrupt line
// Which gets mapped to the first SPI interrupt (physical 32).
const AARCH64_SERIAL_1_3_IRQ: u32 = 0;
const AARCH64_SERIAL_2_4_IRQ: u32 = 2;

// Place the RTC device at page 2
const AARCH64_RTC_ADDR: u64 = 0x2000;
// The RTC device gets one 4k page
const AARCH64_RTC_SIZE: u64 = 0x1000;
// The RTC device gets the second interrupt line
const AARCH64_RTC_IRQ: u32 = 1;

// PCI MMIO configuration region base address.
const AARCH64_PCI_CFG_BASE: u64 = 0x10000;
// PCI MMIO configuration region size.
const AARCH64_PCI_CFG_SIZE: u64 = 0x1000000;
// This is the base address of MMIO devices.
const AARCH64_MMIO_BASE: u64 = 0x1010000;
// Size of the whole MMIO region.
const AARCH64_MMIO_SIZE: u64 = 0x100000;
// Virtio devices start at SPI interrupt number 3
const AARCH64_IRQ_BASE: u32 = 3;

// PMU PPI interrupt, same as qemu
const AARCH64_PMU_IRQ: u32 = 7;

#[sorted]
#[derive(Error, Debug)]
pub enum Error {
    #[error("bios could not be loaded: {0}")]
    BiosLoadFailure(arch::LoadImageError),
    #[error("failed to build arm pvtime memory: {0}")]
    BuildPvtimeError(base::MmapError),
    #[error("unable to clone an Event: {0}")]
    CloneEvent(base::Error),
    #[error("failed to clone IRQ chip: {0}")]
    CloneIrqChip(base::Error),
    #[error("the given kernel command line was invalid: {0}")]
    Cmdline(kernel_cmdline::Error),
    #[error("unable to make an Event: {0}")]
    CreateEvent(base::Error),
    #[error("FDT could not be created: {0}")]
    CreateFdt(arch::fdt::Error),
    #[error("failed to create GIC: {0}")]
    CreateGICFailure(base::Error),
    #[error("failed to create a PCI root hub: {0}")]
    CreatePciRoot(arch::DeviceRegistrationError),
    #[error("failed to create platform bus: {0}")]
    CreatePlatformBus(arch::DeviceRegistrationError),
    #[error("unable to create serial devices: {0}")]
    CreateSerialDevices(arch::DeviceRegistrationError),
    #[error("failed to create socket: {0}")]
    CreateSocket(io::Error),
    #[error("failed to create VCPU: {0}")]
    CreateVcpu(base::Error),
    #[error("vm created wrong kind of vcpu")]
    DowncastVcpu,
    #[error("failed to finalize IRQ chip: {0}")]
    FinalizeIrqChip(base::Error),
    #[error("failed to get PSCI version: {0}")]
    GetPsciVersion(base::Error),
    #[error("failed to get serial cmdline: {0}")]
    GetSerialCmdline(GetSerialCmdlineError),
    #[error("failed to initialize arm pvtime: {0}")]
    InitPvtimeError(base::Error),
    #[error("initrd could not be loaded: {0}")]
    InitrdLoadFailure(arch::LoadImageError),
    #[error("kernel could not be loaded: {0}")]
    KernelLoadFailure(arch::LoadImageError),
    #[error("failed to map arm pvtime memory: {0}")]
    MapPvtimeError(base::Error),
    #[error("failed to protect vm: {0}")]
    ProtectVm(base::Error),
    #[error("ramoops address is different from high_mmio_base: {0} vs {1}")]
    RamoopsAddress(u64, u64),
    #[error("failed to register irq fd: {0}")]
    RegisterIrqfd(base::Error),
    #[error("error registering PCI bus: {0}")]
    RegisterPci(BusError),
    #[error("error registering virtual socket device: {0}")]
    RegisterVsock(arch::DeviceRegistrationError),
    #[error("failed to set device attr: {0}")]
    SetDeviceAttr(base::Error),
    #[error("failed to set register: {0}")]
    SetReg(base::Error),
    #[error("failed to set up guest memory: {0}")]
    SetupGuestMemory(GuestMemoryError),
    #[error("this function isn't supported")]
    Unsupported,
    #[error("failed to initialize VCPU: {0}")]
    VcpuInit(base::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Returns a Vec of the valid memory addresses.
/// These should be used to configure the GuestMemory structure for the platfrom.
pub fn arch_memory_regions(size: u64) -> Vec<(GuestAddress, u64)> {
    vec![(GuestAddress(AARCH64_PHYS_MEM_START), size)]
}

fn fdt_offset(mem_size: u64, has_bios: bool) -> u64 {
    // TODO(rammuthiah) make kernel and BIOS startup use FDT from the same location. ARCVM startup
    // currently expects the kernel at 0x80080000 and the FDT at the end of RAM for unknown reasons.
    // Root cause and figure out how to fold these code paths together.
    if has_bios {
        AARCH64_FDT_OFFSET_IN_BIOS_MODE
    } else {
        // Put fdt up near the top of memory
        // TODO(sonnyrao): will have to handle this differently if there's
        // > 4GB memory
        mem_size - AARCH64_FDT_MAX_SIZE - 0x10000
    }
}

pub struct AArch64;

impl arch::LinuxArch for AArch64 {
    type Error = Error;

    fn guest_memory_layout(
        components: &VmComponents,
    ) -> std::result::Result<Vec<(GuestAddress, u64)>, Self::Error> {
        Ok(arch_memory_regions(components.memory_size))
    }

    fn get_phys_max_addr() -> u64 {
        u64::max_value()
    }

    fn create_system_allocator(guest_mem: &GuestMemory) -> SystemAllocator {
        Self::get_resource_allocator(guest_mem.memory_size())
    }

    fn build_vm<V, Vcpu>(
        mut components: VmComponents,
        _exit_evt: &Event,
        system_allocator: &mut SystemAllocator,
        serial_parameters: &BTreeMap<(SerialHardware, u8), SerialParameters>,
        serial_jail: Option<Minijail>,
        _battery: (&Option<BatteryType>, Option<Minijail>),
        mut vm: V,
        ramoops_region: Option<arch::pstore::RamoopsRegion>,
        devs: Vec<(Box<dyn BusDeviceObj>, Option<Minijail>)>,
        irq_chip: &mut dyn IrqChipAArch64,
        kvm_vcpu_ids: &mut Vec<usize>,
    ) -> std::result::Result<RunnableLinuxVm<V, Vcpu>, Self::Error>
    where
        V: VmAArch64,
        Vcpu: VcpuAArch64,
    {
        let has_bios = match components.vm_image {
            VmImage::Bios(_) => true,
            _ => false,
        };

        let mem = vm.get_memory().clone();

        let mut use_pmu = vm
            .get_hypervisor()
            .check_capability(&HypervisorCap::ArmPmuV3);
        let vcpu_count = components.vcpu_count;
        let mut has_pvtime = true;
        let mut vcpus = Vec::with_capacity(vcpu_count);
        for vcpu_id in 0..vcpu_count {
            let vcpu: Vcpu = *vm
                .create_vcpu(vcpu_id)
                .map_err(Error::CreateVcpu)?
                .downcast::<Vcpu>()
                .map_err(|_| Error::DowncastVcpu)?;
            Self::configure_vcpu_early(
                vm.get_memory(),
                &vcpu,
                vcpu_id,
                use_pmu,
                has_bios,
                components.protected_vm,
            )?;
            has_pvtime &= vcpu.has_pvtime_support();
            vcpus.push(vcpu);
            kvm_vcpu_ids.push(vcpu_id);
        }

        irq_chip.finalize().map_err(Error::FinalizeIrqChip)?;

        if has_pvtime {
            let pvtime_mem = MemoryMappingBuilder::new(AARCH64_PVTIME_IPA_MAX_SIZE as usize)
                .build()
                .map_err(Error::BuildPvtimeError)?;
            vm.add_memory_region(
                GuestAddress(AARCH64_PVTIME_IPA_START),
                Box::new(pvtime_mem),
                false,
                false,
            )
            .map_err(Error::MapPvtimeError)?;
        }

        if components.protected_vm == ProtectionType::Protected {
            vm.enable_protected_vm(
                GuestAddress(AARCH64_PROTECTED_VM_FW_START),
                AARCH64_PROTECTED_VM_FW_MAX_SIZE,
            )
            .map_err(Error::ProtectVm)?;
        }

        for (vcpu_id, vcpu) in vcpus.iter().enumerate() {
            use_pmu &= vcpu.init_pmu(AARCH64_PMU_IRQ as u64 + 16).is_ok();
            if has_pvtime {
                vcpu.init_pvtime(AARCH64_PVTIME_IPA_START + (vcpu_id as u64 * AARCH64_PVTIME_SIZE))
                    .map_err(Error::InitPvtimeError)?;
            }
        }

        let mmio_bus = Arc::new(devices::Bus::new());

        // ARM doesn't really use the io bus like x86, so just create an empty bus.
        let io_bus = Arc::new(devices::Bus::new());

        // Event used by PMDevice to notify crosvm that
        // guest OS is trying to suspend.
        let suspend_evt = Event::new().map_err(Error::CreateEvent)?;

        let (pci_devices, others): (Vec<_>, Vec<_>) = devs
            .into_iter()
            .partition(|(dev, _)| dev.as_pci_device().is_some());

        let pci_devices = pci_devices
            .into_iter()
            .map(|(dev, jail_orig)| (dev.into_pci_device().unwrap(), jail_orig))
            .collect();
        let (pci, pci_irqs, mut pid_debug_label_map) = arch::generate_pci_root(
            pci_devices,
            irq_chip.as_irq_chip_mut(),
            mmio_bus.clone(),
            io_bus.clone(),
            system_allocator,
            &mut vm,
            (devices::AARCH64_GIC_NR_IRQS - AARCH64_IRQ_BASE) as usize,
        )
        .map_err(Error::CreatePciRoot)?;
        let pci_bus = Arc::new(Mutex::new(PciConfigMmio::new(pci)));

        let (platform_devices, _others): (Vec<_>, Vec<_>) = others
            .into_iter()
            .partition(|(dev, _)| dev.as_platform_device().is_some());

        let platform_devices = platform_devices
            .into_iter()
            .map(|(dev, jail_orig)| (*(dev.into_platform_device().unwrap()), jail_orig))
            .collect();
        let mut platform_pid_debug_label_map = arch::generate_platform_bus(
            platform_devices,
            irq_chip.as_irq_chip_mut(),
            &mmio_bus,
            system_allocator,
        )
        .map_err(Error::CreatePlatformBus)?;
        pid_debug_label_map.append(&mut platform_pid_debug_label_map);

        Self::add_arch_devs(irq_chip.as_irq_chip_mut(), &mmio_bus)?;

        let com_evt_1_3 = Event::new().map_err(Error::CreateEvent)?;
        let com_evt_2_4 = Event::new().map_err(Error::CreateEvent)?;
        arch::add_serial_devices(
            components.protected_vm,
            &mmio_bus,
            &com_evt_1_3,
            &com_evt_2_4,
            serial_parameters,
            serial_jail,
        )
        .map_err(Error::CreateSerialDevices)?;

        irq_chip
            .register_irq_event(AARCH64_SERIAL_1_3_IRQ, &com_evt_1_3, None)
            .map_err(Error::RegisterIrqfd)?;
        irq_chip
            .register_irq_event(AARCH64_SERIAL_2_4_IRQ, &com_evt_2_4, None)
            .map_err(Error::RegisterIrqfd)?;

        mmio_bus
            .insert(pci_bus.clone(), AARCH64_PCI_CFG_BASE, AARCH64_PCI_CFG_SIZE)
            .map_err(Error::RegisterPci)?;

        let mut cmdline = Self::get_base_linux_cmdline();
        get_serial_cmdline(&mut cmdline, serial_parameters, "mmio")
            .map_err(Error::GetSerialCmdline)?;
        for param in components.extra_kernel_params {
            cmdline.insert_str(&param).map_err(Error::Cmdline)?;
        }

        let psci_version = vcpus[0].get_psci_version().map_err(Error::GetPsciVersion)?;

        // Use the entire high MMIO except the ramoops region for PCI.
        // Note: This assumes that the ramoops region is the first thing allocated from the high
        //       MMIO region.
        let (high_mmio_base, high_mmio_size) =
            Self::get_high_mmio_base_size(components.memory_size);
        let (pci_device_base, pci_device_size) = match &ramoops_region {
            Some(r) => {
                if r.address != high_mmio_base {
                    return Err(Error::RamoopsAddress(r.address, high_mmio_base));
                }
                arch::pstore::add_ramoops_kernel_cmdline(&mut cmdline, r)
                    .map_err(Error::Cmdline)?;
                let base = r.address + r.size as u64;
                (base, high_mmio_size - (base - high_mmio_base))
            }
            None => (high_mmio_base, high_mmio_size),
        };
        let mut initrd = None;

        // separate out image loading from other setup to get a specific error for
        // image loading
        match components.vm_image {
            VmImage::Bios(ref mut bios) => {
                arch::load_image(&mem, bios, get_bios_addr(), AARCH64_BIOS_MAX_LEN)
                    .map_err(Error::BiosLoadFailure)?;
            }
            VmImage::Kernel(ref mut kernel_image) => {
                let kernel_size =
                    arch::load_image(&mem, kernel_image, get_kernel_addr(), u64::max_value())
                        .map_err(Error::KernelLoadFailure)?;
                let kernel_end = get_kernel_addr().offset() + kernel_size as u64;
                initrd = match components.initrd_image {
                    Some(initrd_file) => {
                        let mut initrd_file = initrd_file;
                        let initrd_addr =
                            (kernel_end + (AARCH64_INITRD_ALIGN - 1)) & !(AARCH64_INITRD_ALIGN - 1);
                        let initrd_max_size =
                            components.memory_size - (initrd_addr - AARCH64_PHYS_MEM_START);
                        let initrd_addr = GuestAddress(initrd_addr);
                        let initrd_size =
                            arch::load_image(&mem, &mut initrd_file, initrd_addr, initrd_max_size)
                                .map_err(Error::InitrdLoadFailure)?;
                        Some((initrd_addr, initrd_size))
                    }
                    None => None,
                };
            }
        }

        fdt::create_fdt(
            AARCH64_FDT_MAX_SIZE as usize,
            &mem,
            pci_irqs,
            vcpu_count as u32,
            components.cpu_clusters,
            components.cpu_capacity,
            fdt_offset(components.memory_size, has_bios),
            pci_device_base,
            pci_device_size,
            cmdline.as_str(),
            initrd,
            components.android_fstab,
            irq_chip.get_vgic_version() == DeviceKind::ArmVgicV3,
            use_pmu,
            psci_version,
            components.swiotlb,
        )
        .map_err(Error::CreateFdt)?;

        Ok(RunnableLinuxVm {
            vm,
            vcpu_count,
            vcpus: Some(vcpus),
            vcpu_affinity: components.vcpu_affinity,
            no_smt: components.no_smt,
            irq_chip: irq_chip.try_box_clone().map_err(Error::CloneIrqChip)?,
            has_bios,
            io_bus,
            mmio_bus,
            pid_debug_label_map,
            suspend_evt,
            rt_cpus: components.rt_cpus,
            delay_rt: components.delay_rt,
            bat_control: None,
            resume_notify_devices: Vec::new(),
            root_config: pci_bus,
            hotplug_bus: None,
        })
    }

    fn configure_vcpu(
        _guest_mem: &GuestMemory,
        _hypervisor: &dyn Hypervisor,
        _irq_chip: &mut dyn IrqChipAArch64,
        _vcpu: &mut dyn VcpuAArch64,
        _vcpu_id: usize,
        _num_cpus: usize,
        _has_bios: bool,
        _no_smt: bool,
        _host_cpu_topology: bool,
    ) -> std::result::Result<(), Self::Error> {
        // AArch64 doesn't configure vcpus on the vcpu thread, so nothing to do here.
        Ok(())
    }

    fn register_pci_device<V: VmAArch64, Vcpu: VcpuAArch64>(
        _linux: &mut RunnableLinuxVm<V, Vcpu>,
        _device: Box<dyn PciDevice>,
        _minijail: Option<Minijail>,
        _resources: &mut SystemAllocator,
    ) -> std::result::Result<PciAddress, Self::Error> {
        // hotplug function isn't verified on AArch64, so set it unsupported here.
        Err(Error::Unsupported)
    }
}

impl AArch64 {
    fn get_high_mmio_base_size(mem_size: u64) -> (u64, u64) {
        let base = AARCH64_PHYS_MEM_START + mem_size + AARCH64_PLATFORM_MMIO_SIZE;
        let size = u64::max_value() - base;
        (base, size)
    }

    fn get_platform_mmio_base_size(mem_size: u64) -> (u64, u64) {
        let base = AARCH64_PHYS_MEM_START + mem_size;
        let size = AARCH64_PLATFORM_MMIO_SIZE;
        (base, size)
    }

    /// This returns a base part of the kernel command for this architecture
    fn get_base_linux_cmdline() -> kernel_cmdline::Cmdline {
        let mut cmdline = kernel_cmdline::Cmdline::new(base::pagesize());
        cmdline.insert_str("panic=-1").unwrap();
        cmdline
    }

    /// Returns a system resource allocator.
    fn get_resource_allocator(mem_size: u64) -> SystemAllocator {
        let (high_mmio_base, high_mmio_size) = Self::get_high_mmio_base_size(mem_size);
        let (plat_mmio_base, plat_mmio_size) = Self::get_platform_mmio_base_size(mem_size);
        SystemAllocator::builder()
            .add_high_mmio_addresses(high_mmio_base, high_mmio_size)
            .add_low_mmio_addresses(AARCH64_MMIO_BASE, AARCH64_MMIO_SIZE)
            .add_platform_mmio_addresses(plat_mmio_base, plat_mmio_size)
            .create_allocator(AARCH64_IRQ_BASE)
            .unwrap()
    }

    /// This adds any early platform devices for this architecture.
    ///
    /// # Arguments
    ///
    /// * `irq_chip` - The IRQ chip to add irqs to.
    /// * `bus` - The bus to add devices to.
    fn add_arch_devs(irq_chip: &mut dyn IrqChip, bus: &Bus) -> Result<()> {
        let rtc_evt = Event::new().map_err(Error::CreateEvent)?;
        irq_chip
            .register_irq_event(AARCH64_RTC_IRQ, &rtc_evt, None)
            .map_err(Error::RegisterIrqfd)?;

        let rtc = Arc::new(Mutex::new(devices::pl030::Pl030::new(rtc_evt)));
        bus.insert(rtc, AARCH64_RTC_ADDR, AARCH64_RTC_SIZE)
            .expect("failed to add rtc device");

        Ok(())
    }

    /// Sets up `vcpu`.
    ///
    /// AArch64 needs vcpus set up before its kernel IRQ chip is created, so `configure_vcpu_early`
    /// is called from `build_vm` on the main thread.  `LinuxArch::configure_vcpu`, which is used
    /// by X86_64 to do setup later from the vcpu thread, is a no-op on AArch64 since vcpus were
    /// already configured here.
    ///
    /// # Arguments
    ///
    /// * `guest_mem` - The guest memory object.
    /// * `vcpu` - The vcpu to configure.
    /// * `vcpu_id` - The VM's index for `vcpu`.
    /// * `use_pmu` - Should `vcpu` be configured to use the Performance Monitor Unit.
    fn configure_vcpu_early(
        guest_mem: &GuestMemory,
        vcpu: &dyn VcpuAArch64,
        vcpu_id: usize,
        use_pmu: bool,
        has_bios: bool,
        protected_vm: ProtectionType,
    ) -> Result<()> {
        let mut features = vec![VcpuFeature::PsciV0_2];
        if use_pmu {
            features.push(VcpuFeature::PmuV3);
        }
        // Non-boot cpus are powered off initially
        if vcpu_id != 0 {
            features.push(VcpuFeature::PowerOff)
        }
        vcpu.init(&features).map_err(Error::VcpuInit)?;

        // All interrupts masked
        let pstate = PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_MODE_EL1H;
        vcpu.set_one_reg(arm64_core_reg!(pstate), pstate)
            .map_err(Error::SetReg)?;

        // Other cpus are powered off initially
        if vcpu_id == 0 {
            let entry_addr = if has_bios {
                get_bios_addr()
            } else {
                get_kernel_addr()
            };
            let entry_addr_reg_id = if protected_vm == ProtectionType::Protected {
                arm64_core_reg!(regs, 1)
            } else {
                arm64_core_reg!(pc)
            };
            vcpu.set_one_reg(entry_addr_reg_id, entry_addr.offset())
                .map_err(Error::SetReg)?;

            /* X0 -- fdt address */
            let mem_size = guest_mem.memory_size();
            let fdt_addr = (AARCH64_PHYS_MEM_START + fdt_offset(mem_size, has_bios)) as u64;
            vcpu.set_one_reg(arm64_core_reg!(regs, 0), fdt_addr)
                .map_err(Error::SetReg)?;
        }

        Ok(())
    }
}
