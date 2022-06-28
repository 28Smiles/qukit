use crate::quantum::operator::simple::controlled::Controlled;
use crate::quantum::operator::simple::hadamard::Hadamard;
use crate::quantum::operator::simple::pauli_x::PauliX;
use crate::quantum::operator::simple::pauli_x_root::PauliXRoot;
use crate::quantum::operator::simple::pauli_y::PauliY;
use crate::quantum::operator::simple::pauli_z::PauliZ;
use crate::quantum::operator::simple::s_gate::SGate;
use crate::quantum::operator::simple::s_gate_inverse::SGateInverse;
use crate::quantum::operator::simple::swap::Swap;
use crate::quantum::operator::simple::swap_root::SwapRoot;
use crate::quantum::operator::simple::t_gate::TGate;
use crate::quantum::operator::traits::{ApplyGate, ApplyGateParameterized, Parameterized, UsedWires};
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::sized::SizedGate;

pub mod controlled;
pub mod hadamard;
pub mod pauli_x;
pub mod pauli_x_root;
pub mod pauli_y;
pub mod pauli_z;
pub mod s_gate;
pub mod s_gate_inverse;
pub mod swap;
pub mod swap_root;
pub mod t_gate;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(tag = "type"))]
#[derive(Copy, Clone, PartialEq)]
pub enum Simple {
    // Primitives
    PauliX(PauliX),
    PauliY(PauliY),
    PauliZ(PauliZ),
    Hadamard(Hadamard),
    TGate(TGate),
    SGate(SGate),
    SGateInverse(SGateInverse),
    PauliXRoot(PauliXRoot),
    Swap(Swap),
    SwapRoot(SwapRoot),
    // Controlled
    ControlledPauliX(Controlled<2, PauliX>),
    ControlledPauliY(Controlled<2, PauliY>),
    ControlledPauliZ(Controlled<2, PauliZ>),
    ControlledHadamard(Controlled<2, Hadamard>),
    ControlledTGate(Controlled<2, TGate>),
    ControlledSGate(Controlled<2, SGate>),
    ControlledSGateInverse(Controlled<2, SGateInverse>),
    ControlledPauliXRoot(Controlled<2, PauliXRoot>),
    ControlledSwap(Controlled<3, Swap>),
    ControlledSwapRoot(Controlled<3, SwapRoot>),
    // Controlled Controlled
    ControlledControlledPauliX(Controlled<3, Controlled<2, PauliX>>),
    ControlledControlledPauliY(Controlled<3, Controlled<2, PauliY>>),
    ControlledControlledPauliZ(Controlled<3, Controlled<2, PauliZ>>),
    ControlledControlledHadamard(Controlled<3, Controlled<2, Hadamard>>),
    ControlledControlledTGate(Controlled<3, Controlled<2, TGate>>),
    ControlledControlledSGate(Controlled<3, Controlled<2, SGate>>),
    ControlledControlledSGateInverse(Controlled<3, Controlled<2, SGateInverse>>),
    ControlledControlledPauliXRoot(Controlled<3, Controlled<2, PauliXRoot>>),
}

impl Simple {
    pub fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        match self {
            Simple::PauliX(gate) => gate.apply_parameterized(theta, computer),
            Simple::PauliY(gate) => gate.apply_parameterized(theta, computer),
            Simple::PauliZ(gate) => gate.apply_parameterized(theta, computer),
            Simple::Hadamard(gate) => gate.apply_parameterized(theta, computer),
            Simple::TGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::SGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::SGateInverse(gate) => gate.apply_parameterized(theta, computer),
            Simple::PauliXRoot(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledPauliX(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledPauliY(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledPauliZ(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledHadamard(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledTGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledSGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledSGateInverse(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledPauliXRoot(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledPauliX(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledPauliY(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledPauliZ(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledHadamard(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledTGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledSGate(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledSGateInverse(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledControlledPauliXRoot(gate) => gate.apply_parameterized(theta, computer),
            Simple::Swap(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledSwap(gate) => gate.apply_parameterized(theta, computer),
            Simple::SwapRoot(gate) => gate.apply_parameterized(theta, computer),
            Simple::ControlledSwapRoot(gate) => gate.apply_parameterized(theta, computer),
        }
    }

    pub fn get_parameterized(&self, theta: f64) -> SizedGate {
        match self {
            Simple::PauliX(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::PauliY(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::PauliZ(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::Hadamard(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::TGate(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SGate(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SGateInverse(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::PauliXRoot(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledPauliX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledPauliY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledPauliZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledHadamard(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledTGate(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledSGate(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledSGateInverse(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledPauliXRoot(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledPauliX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledPauliY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledPauliZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledHadamard(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledTGate(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledSGate(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledSGateInverse(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledControlledPauliXRoot(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::Swap(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledSwap(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SwapRoot(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::ControlledSwapRoot(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
        }
    }

    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            Simple::PauliX(gate) => gate.apply(computer),
            Simple::PauliY(gate) => gate.apply(computer),
            Simple::PauliZ(gate) => gate.apply(computer),
            Simple::Hadamard(gate) => gate.apply(computer),
            Simple::TGate(gate) => gate.apply(computer),
            Simple::SGate(gate) => gate.apply(computer),
            Simple::SGateInverse(gate) => gate.apply(computer),
            Simple::PauliXRoot(gate) => gate.apply(computer),
            Simple::ControlledPauliX(gate) => gate.apply(computer),
            Simple::ControlledPauliY(gate) => gate.apply(computer),
            Simple::ControlledPauliZ(gate) => gate.apply(computer),
            Simple::ControlledHadamard(gate) => gate.apply(computer),
            Simple::ControlledTGate(gate) => gate.apply(computer),
            Simple::ControlledSGate(gate) => gate.apply(computer),
            Simple::ControlledSGateInverse(gate) => gate.apply(computer),
            Simple::ControlledPauliXRoot(gate) => gate.apply(computer),
            Simple::ControlledControlledPauliX(gate) => gate.apply(computer),
            Simple::ControlledControlledPauliY(gate) => gate.apply(computer),
            Simple::ControlledControlledPauliZ(gate) => gate.apply(computer),
            Simple::ControlledControlledHadamard(gate) => gate.apply(computer),
            Simple::ControlledControlledTGate(gate) => gate.apply(computer),
            Simple::ControlledControlledSGate(gate) => gate.apply(computer),
            Simple::ControlledControlledSGateInverse(gate) => gate.apply(computer),
            Simple::ControlledControlledPauliXRoot(gate) => gate.apply(computer),
            Simple::Swap(gate) => gate.apply(computer),
            Simple::ControlledSwap(gate) => gate.apply(computer),
            Simple::SwapRoot(gate) => gate.apply(computer),
            Simple::ControlledSwapRoot(gate) => gate.apply(computer),
        }
    }
}
