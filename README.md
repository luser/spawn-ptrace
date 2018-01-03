[![Build Status](https://travis-ci.org/luser/spawn-ptrace.svg?branch=master)](https://travis-ci.org/luser/spawn-ptrace) [![crates.io](https://img.shields.io/crates/v/spawn-ptrace.svg)](https://crates.io/crates/spawn-ptrace) [![](https://docs.rs/spawn-ptrace/badge.svg)](https://docs.rs/spawn-ptrace)

Execute a child process with ptrace enabled.

# Example

```rust,no_run
extern crate spawn_ptrace;

use std::io;
use spawn_ptrace::CommandPtraceSpawn;
use std::process::Command;

fn run() -> io::Result<()> {
   let child = Command::new("/bin/ls").spawn_ptrace()?;
   // call `ptrace(PTRACE_CONT, child.id(), ...)` to continue execution
   // do other ptrace things here...
   Ok(())
}

fn main() {
  run().unwrap();
}
```

# License

This software is provided under the MIT license. See [LICENSE](LICENSE).
