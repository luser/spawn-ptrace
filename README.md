[![Build Status](https://travis-ci.org/luser/spawn-ptrace.svg?branch=master)](https://travis-ci.org/luser/spawn-ptrace) [![crates.io](https://img.shields.io/crates/v/spawn-ptrace.svg)](https://crates.io/crates/spawn-ptrace)

Execute a child process with ptrace enabled. Currently requires Nightly Rust.

# Example

```rust,no_run
# use std::io;
use spawn_ptrace::CommandPtraceSpawn;
use std::process::Command;

# fn foo() -> io::Result<()> {
let child = try!(Command::new("/bin/ls").spawn_ptrace());
// call `ptrace(PTRACE_CONT, child.id(), ...)` to continue execution
// do other ptrace things here...
# Ok(())
# }
```
