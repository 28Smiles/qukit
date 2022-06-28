[![Npm](https://badgen.net/npm/v/qukit)](https://www.npmjs.com/package/qukit)
[![Crates](https://badgen.net/crates/v/qukit)](https://crates.io/crates/qukit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/28Smiles/qukit/actions/workflows/ci.yml/badge.svg)](https://github.com/28Smiles/qukit/actions/workflows/build.yml)
[![Latest Stable](https://img.shields.io/github/v/release/28Smiles/qukit?label=latest%20stable)](https://github.com/28Smiles/qukit/releases/latest)
[![Latest Release](https://img.shields.io/github/v/release/28Smiles/qukit?include_prereleases&label=latest%20release)](https://github.com/28Smiles/qukit/releases)

# Qukit - Quantum Simulation Toolkit

[Qukit](https://github.com/28Smiles/qukit) is an open source quantum circuit simulator implemented in rust and compiled for wasm. [Qukit](https://github.com/28Smiles/qukit) is capable of running 20+ q-bit simulations in browser or at the server (rust and node.js). You can use it in your javascript program to run quantum simulations.

## Wasm Limitations

In wasm you are limited to 2GB/4GB of memory, this means, with this library you are able to simulate up to 25 q-bits.
For a 25 q-bit system we need to keep track of `2^26` states.
A state is described by a complex value, which is composed of 2 `f64` values, which equates to `2 x 8 = 16 Bytes`.
This equates to a state vector of `2^26 x 16 = 1073731824 Bytes ≈ 1.07 GB`.
For each transformation we need one source and one target vector, this leads to a memory usage of `2.14 GB`.
With a future stabilisation of wasm64 we can simulate large vectors.
