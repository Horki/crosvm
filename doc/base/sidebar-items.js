initSidebarItems({"constant":[["IOCSIZE_MASK",""],["IOCSIZE_MASK",""],["IOCSIZE_SHIFT",""],["IOCSIZE_SHIFT",""],["IOC_IN",""],["IOC_IN",""],["IOC_INOUT",""],["IOC_INOUT",""],["IOC_OUT",""],["IOC_OUT",""],["SCM_SOCKET_MAX_FD_COUNT","The maximum number of FDs that can be sent in a single send."],["_IOC_DIRBITS",""],["_IOC_DIRBITS",""],["_IOC_DIRMASK",""],["_IOC_DIRMASK",""],["_IOC_DIRSHIFT",""],["_IOC_DIRSHIFT",""],["_IOC_NONE",""],["_IOC_NONE",""],["_IOC_NRBITS",""],["_IOC_NRBITS",""],["_IOC_NRMASK",""],["_IOC_NRMASK",""],["_IOC_NRSHIFT",""],["_IOC_NRSHIFT",""],["_IOC_READ",""],["_IOC_READ",""],["_IOC_SIZEBITS",""],["_IOC_SIZEBITS",""],["_IOC_SIZEMASK",""],["_IOC_SIZEMASK",""],["_IOC_SIZESHIFT",""],["_IOC_SIZESHIFT",""],["_IOC_TYPEBITS",""],["_IOC_TYPEBITS",""],["_IOC_TYPEMASK",""],["_IOC_TYPEMASK",""],["_IOC_TYPESHIFT",""],["_IOC_TYPESHIFT",""],["_IOC_WRITE",""],["_IOC_WRITE",""]],"derive":[["PollToken","Implements the PollToken trait for a given `enum`."]],"enum":[["CloneError",""],["CloneNamespace","Controls what namespace `clone_process` will have. See NAMESPACES(7)."],["EventReadResult","Wrapper around the return value of doing a read on an EventFd which distinguishes between getting a valid count of the number of times the eventfd has been written to and timing out waiting for the count to be non-zero."],["EventReadResult","Wrapper around the return value of doing a read on an EventFd which distinguishes between getting a valid count of the number of times the eventfd has been written to and timing out waiting for the count to be non-zero."],["EventType","Represents types of events to watch for."],["ExternalMappingError",""],["FallocateMode","The operation to perform with `fallocate`."],["FileFlags",""],["FlockOperation","The operation to perform with `flock`."],["MmapError",""],["Signal",""],["SignalFdError",""],["TubeError",""],["WaitStatus","A logical set of the values *status can take from libc::wait and libc::waitpid."]],"fn":[["SIGRTMAX","Returns the maximum (inclusive) real-time signal number."],["SIGRTMIN","Returns the minimum (inclusive) real-time signal number."],["add_fd_flags","Performs a logical OR of the given flags with the FD’s flags, setting the given bits for the FD."],["block_signal","Masks given signal."],["chown","Safe wrapper for chown(2)."],["clear_fd_flags","Clears the given flags in the FD’s flags."],["clear_signal","Clears pending signal."],["clear_signal_handler","Resets the signal handler of signum `num` back to the default."],["clone_descriptor","Clones `descriptor`, returning a new `RawDescriptor` that refers to the same open file description as `descriptor`. The cloned descriptor will have the `FD_CLOEXEC` flag set but will not share any other file descriptor flags with `descriptor`."],["clone_fd","Clones `fd`, returning a new file descriptor that refers to the same open file description as `fd`. The cloned fd will have the `FD_CLOEXEC` flag set but will not share any other file descriptor flags with `fd`."],["clone_process","Clones this process and calls a closure in the new process."],["create_sigset","Creates `sigset` from an array of signal numbers."],["deserialize_with_descriptors","Allows the use of any serde deserializer within a closure while providing access to the a set of descriptors for use in `deserialize_descriptor`."],["drop_capabilities","Drops all capabilities (permitted, inheritable, and effective) from the current process."],["duration_to_timespec","Return a timespec filed with the specified Duration `duration`."],["enable_core_scheduling","Enable experimental core scheduling for the current thread."],["errno_result","Returns the last errno as a Result that is always an error."],["fallocate","Safe wrapper for `fallocate()`."],["fchmod","Safe wrapper for fchmod(2)."],["fchown","Safe wrapper for fchown(2)."],["flock","Safe wrapper for flock(2) with the operation `op` and optionally `nonblocking`. The lock will be dropped automatically when `file` is dropped."],["get_blocked_signals","Retrieves the signal mask of the current thread as a vector of c_ints."],["get_filesystem_type","Obtain file system type of the file system that the file is served from."],["get_max_open_files","Get the max number of open files allowed by the environment."],["getegid","Safe wrapper for `getegid(2)`."],["geteuid","Safe wrapper for `geteuid(2)`."],["getpid","This bypasses `libc`’s caching `getpid(2)` wrapper which can be invalid if a raw clone was used elsewhere."],["getsid","Safe wrapper for `getsid(2)`."],["gettid","Safe wrapper for the gettid Linux systemcall."],["has_default_signal_handler","Returns true if the signal handler for signum `num` is the default."],["ioctl","Run an ioctl with no arguments."],["ioctl","Run an ioctl with no arguments."],["ioctl","Run an ioctl with no arguments."],["ioctl_with_mut_ptr","Run an ioctl with a mutable raw pointer."],["ioctl_with_mut_ptr","Run an ioctl with a mutable raw pointer."],["ioctl_with_mut_ptr","Run an ioctl with a mutable raw pointer."],["ioctl_with_mut_ref","Run an ioctl with a mutable reference."],["ioctl_with_mut_ref","Run an ioctl with a mutable reference."],["ioctl_with_mut_ref","Run an ioctl with a mutable reference."],["ioctl_with_ptr","Run an ioctl with a raw pointer."],["ioctl_with_ptr","Run an ioctl with a raw pointer."],["ioctl_with_ptr","Run an ioctl with a raw pointer."],["ioctl_with_ref","Run an ioctl with an immutable reference."],["ioctl_with_ref","Run an ioctl with an immutable reference."],["ioctl_with_ref","Run an ioctl with an immutable reference."],["ioctl_with_val","Run an ioctl with a single value argument."],["ioctl_with_val","Run an ioctl with a single value argument."],["ioctl_with_val","Run an ioctl with a single value argument."],["iov_max","Safe wrapper for `sysconf(_SC_IOV_MAX)`."],["kernel_has_memfd","Checks if the kernel we are running on has memfd_create. It was introduced in 3.17. Only to be used from tests to prevent running on ancient kernels that won’t support the functionality anyways."],["kill","Safety"],["kill_process_group","Kill all processes in the current process group."],["kill_tree","Terminates and reaps a child process. If the child process is a group leader, its children will be terminated and reaped as well. After the given timeout, the child process and any relevant children are killed (i.e. sent SIGKILL)."],["max_timeout","Return the maximum Duration that can be used with libc::timespec."],["new_pipe_full","Test-only function used to create a pipe that is full. The pipe is created, has its size set to the minimum and then has that much data written to it. Use `new_pipe_full` to test handling of blocking `write` calls in unit tests."],["open_file","Open the file with the given path, or if it is of the form `/proc/self/fd/N` then just use the file descriptor."],["pagesize","Safe wrapper for `sysconf(_SC_PAGESIZE)`."],["pipe","Spawns a pipe pair where the first pipe is the read end and the second pipe is the write end."],["poll_in","Utility function that returns true if the given FD is readable without blocking."],["read_raw_stdin","Read raw bytes from stdin."],["reap_child","Reaps a child process that has terminated."],["register_rt_signal_handler","Registers `handler` as the signal handler for the real-time signal with signum `num`."],["register_signal_handler","Registers `handler` as the signal handler of signum `num`."],["round_up_to_page_size","Uses the system’s page size in bytes to round the given value up to the nearest page boundary."],["safe_descriptor_from_path","If the given path is of the form /proc/self/fd/N for some N, returns `Ok(Some(N))`. Otherwise returns `Ok(None`)."],["set_cpu_affinity","Set the CPU affinity of the current thread to a given set of CPUs."],["set_pipe_size","Sets the pipe signified with fd to `size`."],["set_rt_prio_limit","Enables real time thread priorities in the current thread up to `limit`."],["set_rt_round_robin","Sets the current thread to be scheduled using the round robin real time class with `priority`."],["setsid","Wrapper for `setsid(2)`."],["unblock_signal","Unmasks given signal."],["validate_raw_descriptor","Verifies that |raw_descriptor| is actually owned by this process and duplicates it to ensure that we have a unique handle to it."],["validate_raw_fd","Verifies that |raw_fd| is actually owned by this process and duplicates it to ensure that we have a unique handle to it."],["wait_for_interrupt","Blocks until SIGINT is received, which often happens because Ctrl-C was pressed in an interactive terminal."],["wait_for_pid","A safe wrapper around waitpid."],["wait_for_signal","Wait for signal before continuing. The signal number of the consumed signal is returned on success. EAGAIN means the timeout was reached."],["wrap_descriptor","Wraps an AsRawDescriptor in the simple Descriptor struct, which has AsRawFd methods for interfacing with sys_util"]],"macro":[["debug","A macro for logging debug information."],["error","A macro for logging an error."],["handle_eintr","Macro that retries the given expression every time its result indicates it was interrupted (i.e. returned `EINTR`). This is useful for operations that are prone to being interrupted by signals, such as blocking syscalls."],["handle_eintr_errno","Macro that retries the given expression every time its result indicates it was interrupted. It is intended to use with system functions that signal error by returning `-1` and setting `errno` to appropriate error code (`EINTR`, `EINVAL`, etc.) Most of standard non-reentrant libc functions use this way of signalling errors."],["handle_eintr_rc","Macro that retries the given expression every time its result indicates it was interrupted. It is intended to use with system functions that return `EINTR` and other error codes directly as their result. Most of reentrant functions use this way of signalling errors."],["info","A macro for logging info."],["ioctl_expr","Raw macro to declare the expression that calculates an ioctl number"],["ioctl_io_nr","Declare an ioctl that transfers no data."],["ioctl_ioc_nr","Raw macro to declare a function that returns an ioctl number."],["ioctl_ior_nr","Declare an ioctl that reads data."],["ioctl_iow_nr","Declare an ioctl that writes data."],["ioctl_iowr_nr","Declare an ioctl that reads and writes data."],["log","A macro for logging at an arbitrary priority level."],["syscall",""],["volatile_at_impl",""],["volatile_at_impl",""],["volatile_impl",""],["volatile_impl",""],["warn","A macro for logging a warning."]],"mod":[["file_traits",""],["handle_eintr","Macro and helper trait for handling interrupted routines."],["ioctl","Macros and wrapper functions for dealing with ioctls."],["net",""],["rand","Rust implementation of functionality parallel to libchrome’s base/rand_util.h."],["read_dir",""],["scoped_path",""],["scoped_signal_handler","Provides a struct for registering signal handlers that get cleared on drop."],["signal",""],["syslog","Facilities for sending log message to syslog."],["vsock",""],["with_as_descriptor","Module that exports `serialize`/`deserialize` functions for use with `#[serde(with = \"...\")]` attribute."],["with_raw_descriptor","Module that exports `serialize`/`deserialize` functions for use with `#[serde(with = \"...\")]` attribute. It only works with fields with `RawDescriptor` type."]],"struct":[["AsyncTube",""],["Clock",""],["Descriptor","For use cases where a simple wrapper around a RawDescriptor is needed. This is a simply a wrapper and does not manage the lifetime of the descriptor. Most usages should prefer SafeDescriptor or using a RawDescriptor directly"],["DescriptorAdapter","To use an IO struct with cros_async, the type must be marked with DescriptorIntoAsync (to signify it is suitable for use with async operations), and then wrapped with this type."],["EmptySignalHandler","A signal handler that does nothing."],["EpollContext","EpollContext wraps linux epoll. It provides similar interface to PollContext. It is thread safe while PollContext is not. It requires user to pass in a reference of EpollEvents while PollContext does not. Always use PollContext if you don’t need to access the same epoll from different threads."],["EpollEvents","EpollEvents wraps raw epoll_events, it should only be used with EpollContext."],["Error","An error number, retrieved from errno (man 3 errno), set by a libc function that returned an error."],["Event","See EventFd for struct- and method-level documentation."],["EventFd","A safe wrapper around a Linux eventfd (man 2 eventfd)."],["ExternalMapping","ExternalMapping wraps an external library mapping.  This is useful in cases where where the device memory is not compatible with the mmap interface, such as Vulkan VkDeviceMemory in the non-exportable case or when exported as an opaque fd."],["FakeClock","A fake clock that can be used in tests to give exact control over the time. For a code example, see the tests in sys_util/src/timerfd.rs."],["FakeTimer","See FakeTimerFd for struct- and method-level documentation."],["FakeTimerFd","FakeTimerFd: For use in tests."],["Fd",""],["FileSerdeWrapper","A simple wrapper around `File` that implements `Serialize`/`Deserialize`, which is useful when the `#[serde(with = \"with_as_descriptor\")]` trait is infeasible, such as for a field with type `Option<File>`."],["KillOnDrop","Wraps a Child process, and calls kill_tree for its process group to clean it up when dropped."],["LayoutAllocation","A contiguous memory allocation with a specified size and alignment, with a Drop impl to perform the deallocation."],["MemfdSeals","A set of memfd seals."],["MemoryMapping","See MemoryMapping for struct- and method-level documentation."],["MemoryMapping","Wraps an anonymous shared memory mapping in the current process."],["MemoryMappingArena","Tracks Fixed Memory Maps within an anonymous memory-mapped fixed-sized arena in the current process."],["MemoryMappingBuilder",""],["PollContext","Used to poll multiple objects that have file descriptors."],["PollEvent","An event returned by `PollContext::wait`."],["PollEventIter","An iterator over some (sub)set of events returned by `PollContext::wait`."],["PollEvents","The list of event returned by `PollContext::wait`."],["PollEventsOwned","A deep copy of the event records from `PollEvents`."],["Protection","Memory access type for anonymous shared memory mapping."],["SafeDescriptor","Wraps a RawDescriptor and safely closes it when self falls out of scope."],["ScopedEvent","An `EventFd` wrapper which triggers when it goes out of scope."],["ScopedEvent","See ScopedEvent for struct- and method-level documentation."],["ScopedSignalHandler","Represents a signal handler that is registered with a set of signals that unregistered when the struct goes out of scope. Prefer a signalfd based solution before using this."],["SerializeDescriptors","Wrapper for a `Serialize` value which will capture any descriptors exported by the value when given to an ordinary `Serializer`."],["SharedMemory","A shared memory file descriptor and its size."],["SharedMemory","See SharedMemory for struct- and method-level documentation."],["SignalFd","A safe wrapper around a Linux signalfd (man 2 signalfd)."],["Timer","See TimerFd for struct- and method-level documentation."],["TimerFd","A safe wrapper around a Linux timerfd (man 2 timerfd_create)."],["TriggeredEvent","Represents an event that has been signaled and waited for via a wait function."],["Tube","Bidirectional tube that support both send and recv."],["UnlinkUnixDatagram","Used to attempt to clean up a named pipe after it is no longer used."],["UnlinkUnixListener","Used to attempt to clean up a named pipe after it is no longer used."],["WaitContext","Used to wait for multiple objects which are eligible for waiting."],["WatchingEvents","Watching events taken by PollContext."]],"trait":[["AsIobuf","Trait for types that can be converted into an `iovec` that can be referenced by a syscall for the lifetime of this object."],["AsRawDescriptor","Trait for returning the underlying raw descriptor, without giving up ownership of the descriptor."],["AsRawDescriptors","A trait similar to `AsRawDescriptor` but supports an arbitrary number of descriptors."],["AsRawFds","A trait similar to `AsRawFd` but supports an arbitrary number of file descriptors."],["AsRawPid","A trait used to abstract types that provide a process id that can be operated on."],["DescriptorIntoAsync","Like `cros_async::IntoAsync`, except for use with crosvm’s AsRawDescriptor trait object family."],["EventToken",""],["FileAllocate","A trait for allocating disk space in a sparse file. This is equivalent to fallocate() with no special flags."],["FileAllocate","A trait for allocating disk space in a sparse file. This is equivalent to fallocate() with no special flags."],["FileGetLen","A trait for getting the size of a file. This is equivalent to File’s metadata().len() method, but wrapped in a trait so that it can be implemented for other types."],["FileGetLen","A trait for getting the size of a file. This is equivalent to File’s metadata().len() method, but wrapped in a trait so that it can be implemented for other types."],["FileReadWriteAtVolatile","A trait similar to the unix `ReadExt` and `WriteExt` traits, but for volatile memory."],["FileReadWriteAtVolatile","A trait similar to the unix `ReadExt` and `WriteExt` traits, but for volatile memory."],["FileReadWriteVolatile","A trait similar to `Read` and `Write`, but uses volatile memory as buffers."],["FileReadWriteVolatile","A trait similar to `Read` and `Write`, but uses volatile memory as buffers."],["FileSetLen","A trait for setting the size of a file. This is equivalent to File’s `set_len` method, but wrapped in a trait so that it can be implemented for other types."],["FileSetLen","A trait for setting the size of a file. This is equivalent to File’s `set_len` method, but wrapped in a trait so that it can be implemented for other types."],["FileSync","A trait for flushing the contents of a file to disk. This is equivalent to File’s `sync_all` method, but wrapped in a trait so that it can be implemented for other types."],["FileSync","A trait for flushing the contents of a file to disk. This is equivalent to File’s `sync_all` method, but wrapped in a trait so that it can be implemented for other types."],["FromRawDescriptor",""],["IntoRawDescriptor","Trait for forfeiting ownership of the current raw descriptor, and returning the raw descriptor"],["Killable","Trait for threads that can be signalled via `pthread_kill`."],["MappedRegion","A range of memory that can be msynced, for abstracting over different types of memory mappings."],["MemoryMappingBuilderUnix",""],["MemoryMappingUnix",""],["PollToken","Trait for a token that can be associated with an `fd` in a `PollContext`."],["PunchHole","A trait for deallocating space in a file."],["ScmSocket","Trait for file descriptors can send and receive socket control messages via `sendmsg` and `recvmsg`."],["SeekHole","A trait for seeking to the next hole or non-hole position in a file."],["SeekHole","A trait for seeking to the next hole or non-hole position in a file."],["SharedMemoryUnix",""],["SignalHandler","The interface used by Scoped Signal handler."],["Terminal","Trait for file descriptors that are TTYs, according to `isatty(3)`."],["WriteZeroes","A trait for writing zeroes to a stream."],["WriteZeroesAt","A trait for writing zeroes to an arbitrary position in a file."],["WriteZeroesAt","A trait for writing zeroes to an arbitrary position in a file."]],"type":[["ExternalMappingResult",""],["Gid",""],["IoctlNr",""],["IoctlNr",""],["Map",""],["Mode",""],["Pid","Re-export libc types that are part of the API."],["RawDescriptor",""],["Result",""],["SignalResult",""],["TubeResult",""],["Uid",""],["Unmap",""],["UnsyncMarker","Used to mark types as !Sync."]]});