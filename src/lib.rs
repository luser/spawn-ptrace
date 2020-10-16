//! This crate allows you to spawn a child process with [`ptrace`] enabled.
//! It provides a single trait—[`CommandPtraceSpawn`]—that is implemented for
//! `std::process::Command`, giving you access to a [`spawn_ptrace`] method.
//!
//! Processes spawned this way will be stopped with `SIGTRAP` from their
//! `exec`, so you can perform any early intervention you require prior to the
//! process running any code and then use `PTRACE_CONT` to resume its execution.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use std::io;
//! use spawn_ptrace::CommandPtraceSpawn;
//! use std::process::Command;
//!
//! # fn foo() -> io::Result<()> {
//! let child = Command::new("/bin/ls").spawn_ptrace()?;
//! // call `ptrace(PTRACE_CONT, child.id(), ...)` to continue execution
//! // do other ptrace things here...
//! # Ok(())
//! # }
//! ```
//!
//! [`ptrace`]: https://man7.org/linux/man-pages/man2/ptrace.2.html
//! [`CommandPtraceSpawn`]: trait.CommandPtraceSpawn.html
//! [`spawn_ptrace`]: trait.CommandPtraceSpawn.html#tymethod.spawn_ptrace
#![cfg(unix)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;
use std::io::{self, Result};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};

/// A Unix-specific extension to `std::process::Command` to spawn a process with `ptrace` enabled.
///
/// See [the crate-level documentation](index.html) for an example.
pub trait CommandPtraceSpawn {
    /// Executes the command as a child process, also enabling ptrace on it.
    ///
    /// The child process will be stopped with a `SIGTRAP` calling `exec`
    /// to execute the specified command. You can continue it with
    /// `PTRACE_CONT`.
    fn spawn_ptrace(&mut self) -> Result<Child>;
}

impl CommandPtraceSpawn for Command {
    fn spawn_ptrace(&mut self) -> Result<Child> {
        let child = unsafe {
            self.pre_exec(|| {
                // Opt-in to ptrace.
                ptrace::traceme().map_err(|e| match e {
                    nix::Error::Sys(e) => io::Error::from_raw_os_error(e as i32),
                    _ => io::Error::new(io::ErrorKind::Other, "unknown PTRACE_TRACEME error"),
                })
            })
            .spawn()?
        };
        // Ensure that the child is stopped in exec before returning.
        match waitpid(Some(Pid::from_raw(child.id() as i32)), None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => Ok(child),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Child state not correct",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::path::PathBuf;

    fn test_process_path() -> Option<PathBuf> {
        env::current_exe().ok().and_then(|p| {
            p.parent().map(|p| {
                p.with_file_name("test")
                    .with_extension(env::consts::EXE_EXTENSION)
            })
        })
    }

    #[test]
    fn test_spawn_ptrace() {
        let path = test_process_path().expect("Failed to get test process path");
        let child = Command::new(&path)
            .spawn_ptrace()
            .expect("Error spawning test process");
        let pid = Pid::from_raw(child.id() as i32);
        // Let the child continue.
        ptrace::cont(pid, None).expect("Error continuing child process");
        // Wait for the child to exit.
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => assert_eq!(code, 0),
            Ok(s) => panic!("Unexpected stop status: {:?}", s),
            Err(e) => panic!("Unexpected waitpid error: {:?}", e),
        }
    }
}
