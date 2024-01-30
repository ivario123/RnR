# Rust in Rust (RnR)

This crate defines a toy example of a rust clone language.
The language is statically typed and references are checked at compile-time.
The language has no support for user defined types, although it would not be 
that difficult to implement they were left out due to time restrictions.
As for traits and modules they were considered beyond the scope of the
project and will likely never be implemented.

## What works

- [x] Partial Mips backend.
- [x] [`Borrow checking`](./BORROW_CHECKER.md) of both immutable and mutable data.
- [x] Type checking
- [x] A VM for the supported instructions.

