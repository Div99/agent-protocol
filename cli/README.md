# Agent Protocol CLI

## Introduction

This is a cross-platform CLI tool for the agent protocol.

## Execution

Prerequisites: make sure `cargo` and `rust` are available.

To execute the tool, run:

```rust
cargo build
cargo run -h
```

## Formatting

More about code formatting, see [rustfmt](https://github.com/rust-lang/rustfmt)

```rust
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
```

Updates: let's just use the default rust format settings by running `cargo fmt`

## The AP API

Questions:
1. How to visualize the Spec?
1. Shall we modualize the Spec?

## Generator Script

The generator script has some issue. 

For instance, the `TaskRequestBody` has a field called `input` and it is optional.
However, after coversion, its type becomes `Option<Option<String>>`