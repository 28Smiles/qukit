[![Npm](https://badgen.net/npm/v/qukit)](https://www.npmjs.com/package/qukit)
[![Crates](https://badgen.net/crates/v/qukit)](https://crates.io/crates/qukit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/28Smiles/qukit/actions/workflows/ci.yml/badge.svg)](https://github.com/28Smiles/qukit/actions/workflows/build.yml)
[![Latest Stable](https://img.shields.io/github/v/release/28Smiles/qukit?label=latest%20stable)](https://github.com/28Smiles/qukit/releases/latest)
[![Latest Release](https://img.shields.io/github/v/release/28Smiles/qukit?include_prereleases&label=latest%20release)](https://github.com/28Smiles/qukit/releases)

# Qukit - Quantum Simulation Toolkit

[Qukit](https://github.com/28Smiles/qukit) is an open source quantum circuit simulator implemented in rust and compiled for wasm. [Qukit](https://github.com/28Smiles/qukit) is capable of running 20+ q-bit simulations in browser or at the server (rust and node.js). You can use it in your javascript program to run quantum simulations.

## Features
 - [x] Rust API
 - [x] JS API (Wasm)
 - [x] Algorithm creation and simulation
 - [x] Execute an algoritm step wise
 - [x] Convert all supported gates into rotation to enable partial simulation
 - [ ] Python API
 - [ ] QASM Export
 - [ ] SVG Export
 - [ ] State Visulisations
 - [ ] Quiskit Export
 - [ ] QASM Import

## Usage

### TS/JS

```
npm install qukit
```

You should be able to import [Qukit](https://github.com/28Smiles/qukit) directly into Node, as normal, or into a browser using any bundler that supports ES modules & webassembly (e.g. Webpack v4 or v5).

The browser build supports both sync (v4 or v5 syncWebAssembly mode) and async (v5 asyncWebAssembly) builds. When imported in a browser build the module always exports a promise, not a fixed value, as this is a requirement for synchronous builds, and you will need to `await` this after import.

We also provide a parallel version via the [Web Workers API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API), the implementation uses [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon), for further information on setup visit the [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon) github page.

### Rust

First of all, add this crate as a dependency to your `Cargo.toml`:

```toml
[dependencies]
qukit = "0.0.0-pre4"
```

To use this crate u need to use the nighty toolchain, since it heavily uses the latest nightly `const fn` features.

#### Feature Flags

 - `std` Links against std
 - `parallel` enables rayon usage

## Api

### TS/JS

```ts
const builder: GateBuilder = new GateBuilder();
const qbits: QBit[] = builder.qbits(hidden.length);
const bits: Bit[] = builder.bits(hidden.length);
const target: QBit = builder.qbit();

hadamard(target);
pauliZ(target);

hadamard(qbits);

hidden.forEach((active, index) => {
    if (active) {
        cPauliX(qbits[index], target);
    }
});

hadamard(qbits);
hadamard(target);

measurement(qbits, bits);

builder.intoAlgorithm().run(); // -> Executes the Algorithm
```

### Rust

```rust
let algorithm = Algorithm::new(|gate_builder| {
    let a = gate_builder.qbit();
    let b = gate_builder.qbit();
    let c_a = gate_builder.bit();
    let c_b = gate_builder.bit();

    hadamard(a);
    controlled_pauli_x(a, b);

    measurement(a, c_a);
    measurement(b, c_b);

    gate_builder
});

algorithm.run() // -> Executes the Algorithm
```

## Wasm Limitations

In wasm you are limited to 2GB/4GB of memory, thus your are only able to simulate up to 25 q-bits with this library.
For a 25 q-bit system we need to keep track of `2^26` states.
A state is described by a complex value, which is composed of 2 `f64` values, which equates to `2 x 8 = 16 Bytes`.
This equates to a state vector of `2^26 x 16 = 1073731824 Bytes ≈ 1.07 GB`.
For each transformation we need one source and one target vector, this leads to a memory usage of `2.14 GB`.
With a future stabilisation of wasm64 we can simulate large vectors.
