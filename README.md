[![Build Status](https://travis-ci.org/luser/spawn-ptrace.svg?branch=master)](https://travis-ci.org/luser/spawn-ptrace) [![crates.io](https://img.shields.io/crates/v/spawn-ptrace.svg)](https://crates.io/crates/spawn-ptrace) [![](https://docs.rs/spawn-ptrace/badge.svg)](https://docs.rs/spawn-ptrace)

This crate allows you to spawn a child process with [`ptrace`](https://man7.org/linux/man-pages/man2/ptrace.2.html) enabled. It provides a single trait—`CommandPtraceSpawn`—that is implemented for `std::process::Command`, giving you access to a `spawn_ptrace` method.

Processes spawned this way will be stopped with `SIGTRAP` from their `exec`, so you can perform any early intervention you require prior to the process running any code and then use `PTRACE_CONT` to resume its execution.

# Example

```rust,no_run
use std::io;
use spawn_ptrace::CommandPtraceSpawn;
use std::process::Command;

fn main() -> io::Result<()> {
   let child = Command::new("/bin/ls").spawn_ptrace()?;
   // call `ptrace(PTRACE_CONT, child.id(), ...)` to continue execution
   // do other ptrace things here...
   Ok(())
}
```

For a practical example of this crate's usage, see my [`tracetree`](https://github.com/luser/tracetree) tool.

# License

This software is provided under the MIT license. See [LICENSE](LICENSE).
