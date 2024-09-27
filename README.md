# ppc750cl [![Build Status]][actions] [![Latest Version]][crates.io] [![Api Rustdoc]][rustdoc] ![Rust Version]

[Build Status]: https://github.com/encounter/ppc750cl/actions/workflows/test.yml/badge.svg
[actions]: https://github.com/encounter/ppc750cl/actions
[Latest Version]: https://img.shields.io/crates/v/ppc750cl.svg
[crates.io]: https://crates.io/crates/ppc750cl
[Api Rustdoc]: https://img.shields.io/badge/api-rustdoc-blue.svg
[rustdoc]: https://docs.rs/ppc750cl
[Rust Version]: https://img.shields.io/badge/rust-1.74+-blue.svg?maxAge=3600

Rust tools for working with the PowerPC 750CL / 750CXe family of processors.

### Building

```shell
cargo run --package ppc750cl-genisa
cargo build --release
```

### Instruction Set

For those unfamiliar with PowerPC, here are some basics.
- PowerPC 7xx is a family of RISC CPUs produced from 1997 to 2012.
  - They operate with 32-bit words and every instruction is 32-bits wide.
- This project focuses (only) on compatibility with the PowerPC 750CL and 750CXe.
  - PowerPC 750CL ("Broadway") is used in the Nintendo Wii.
  - PowerPC 750CXe ("Gekko") is used in the Nintendo GameCube.
  - They add a "paired-singles" SIMD unit and a bunch of other instructions.

### isa.yaml

The file [isa.yaml](./isa.yaml) contains a full definition of the PowerPC 750CL / 750CXe instruction set.

It powers the disassembler and assembler.

Similarly to LLVM TableGen, the program `ppc750cl-genisa` generates a Rust file implementing an instruction decoder.

### Safety & Correctness

- The disassembler has been fuzzed over all ~4.29 billion possible instructions (via `ppc750cl-fuzz`).
- It is safe to run the disassembler over untrusted byte arrays.
- However, no guarantees on correctness are made (yet). Expect bugs.

### Performance

With a single thread on Ryzen 9 3900X:

- Disassembling & printing: ~12M insn/s (~50 MB/s)
- Disassembling only: ~275M insn/s (~1 GB/s)
