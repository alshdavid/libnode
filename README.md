# Libnode with a C API and Rust bindings

This repo contains patches to Nodejs to add a C API to libnode and a Rust crate that has bindings to embed Nodejs within a Rust application.

## Usage (Rust)

Using the `libnode_rs` crate you can execute JavaScript directly in your Rust application

```rust
use libnode_rs;

pub fn main() -> std::io::Result<()> {
  libnode_rs::eval_blocking(r"
    console.log('[start] Nodejs')
    setTimeout(() => console.log('[end]   Nodejs'), 5000)
  ")
}
```

## Building

```bash
# Linux
./scripts/build

# Windows
./scripts/build.ps1
```

## To Do

- Build libnode statically and vendor it into the Rust crate
  - The expectation is that consumers should be able to produce portable single binary applications with Nodejs embedded
  - If anyone is good with C/C++, linkers, and all of that magic, please help. This is not my area of expertise and I've had no success in my attempts so far
- Support for injecting native functions as Nodejs globals
- Support for interacting with Nodejs `node:worker_threads` 
  - Such as injecting native functions as globals