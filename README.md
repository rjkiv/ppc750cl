# ppc750cl

Rust tools for working with the PowerPC 750CL family of processors.

### Rust crates

```shell
cargo run --package ppc750cl-genisa
cargo build --release
```

### Instruction Set

For those unfamiliar with PowerPC, here are some basics.
- PowerPC 7xx is a family of RISC CPUs produced from 1997 to 2012.
  - They operate with 32-bit words and every instruction is 32-bits wide.
- This project focuses (only) on compatibility with the PowerPC 750CL.
  - This chip is famously packaged as codename "Broadway" for the Nintendo Wii.
  - Its predecessor PowerPC 750CXe is used in the Nintendo GameCube.
  - It adds a "paired-singles" SIMD unit and a bunch of other instructions.

### isa.yaml

The file [isa.yaml](./isa.yaml) contains a full definition of the PowerPC 750CL instruction set.

It powers the disassembler and assembler.

Similarly to LLVM TableGen, the program `ppc750cl-genisa` generates a Rust file implementing an instruction decoder.

### Safety & Correctness

- This project does not use `unsafe` Rust code outside of testing utils.
- The disassembler has been fuzzed over all ~4.29 billion possible instructions (via `ppc750cl-fuzz`).
- It is safe to run the disassembler over untrusted byte arrays.
- However, no guarantees on correctness are made (yet). Expect bugs.

### Performance

With a single thread on Ryzen 9 3900X:

- Disassembling & printing: ~5M insn/s (~20 MB/s)
- Disassembling only: ~50M insn/s (~200 MB/s)
