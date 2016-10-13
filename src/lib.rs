//! Execute a child process with ptrace enabled.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use std::io;
//! use spawn_ptrace::CommandPtraceSpawn;
//! use std::process::Command;
//!
//! # fn foo() -> io::Result<()> {
//! let child = try!(Command::new("/bin/ls").spawn_ptrace());
//! // call `ptrace(PTRACE_CONT, child.id(), ...)` to continue execution
//! // do other ptrace things here...
//! # Ok(())
//! # }
//! ```
#![cfg(unix)]
#![feature(process_exec)]

extern crate nix;

use nix::sys::ptrace::ptrace;
use nix::sys::ptrace::ptrace::PTRACE_TRACEME;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use std::io::{self, Result};
use std::os::unix::process::CommandExt;
use std::process::{Command, Child};
use std::ptr;

/// A Unix-specific extension to `std::process::Command` to spawn a process with `ptrace` enabled.
pub trait CommandPtraceSpawn {
    /// Executes the command as a child process, also enabling ptrace on it.
    ///
    /// The child process will be stopped with a `SIGTRAP` calling `exec`
    /// to execute the specified command. You can continue it with
    /// `PTRACE_CONT`.
    fn spawn_ptrace(self) -> Result<Child>;
}

impl CommandPtraceSpawn for Command {
    fn spawn_ptrace(mut self) -> Result<Child> {
        let child = try!(self.before_exec(|| {
            // Opt-in to ptrace.
            try!(ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut()));
            Ok(())
        })
            .spawn());
        println!("spawned child");
        // Ensure that the child is stopped in exec before returning.
        match waitpid(child.id() as i32, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => Ok(child),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Child state not correct"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use nix::sys::ptrace::ptrace;
    use nix::sys::ptrace::ptrace::PTRACE_CONT;
    use nix::sys::wait::{waitpid, WaitStatus};

    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    use std::ptr;

    fn test_process_path() -> Option<PathBuf> {
        env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.with_file_name("test")
                                         .with_extension(env::consts::EXE_EXTENSION)))
    }

    #[test]
    fn test_spawn_ptrace() {
        let path = test_process_path().expect("Failed to get test process path");
        let child = Command::new(&path)
            .spawn_ptrace()
            .expect("Error spawning test process");
        let pid = child.id() as i32;
        // Let the child continue.
        ptrace(PTRACE_CONT, pid, ptr::null_mut(), ptr::null_mut())
            .expect("Error continuing child process");
        // Wait for the child to exit.
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => assert_eq!(code, 0),
            Ok(s) => panic!("Unexpected stop status: {:?}", s),
            Err(e) => panic!("Unexpected waitpid error: {:?}", e),
        }
    }
}
