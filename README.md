rust-kernel
===========

A kernel written in Rust

Build instructions
------------------

Obtain libcore:

- Download rust source code
- Go to src/libcore
- `rustc --target=i386-unknown-linux lib.rs`
- Copy the .rlib to the current directory

Compile:

- `make`

Run:

- `make run`
