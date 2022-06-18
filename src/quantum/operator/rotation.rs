use crate::quantum::operator::traits::{ApplyGate, Parameterized, UsedWires, ApplyGateParameterized};
use crate::quantum::operator::rotation::rotation_hadamard::RotationHadamard;
use crate::quantum::operator::rotation::rotation_pauli_x::RotationPauliX;
use crate::quantum::operator::rotation::rotation_pauli_y::RotationPauliY;
use crate::quantum::operator::rotation::rotation_pauli_z::RotationPauliZ;
use crate::quantum::operator::rotation::rotation_swap::RotationSwap;
use crate::quantum::operator::rotation::rotation_x::RotationX;
use crate::quantum::operator::rotation::rotation_y::RotationY;
use crate::quantum::operator::rotation::rotation_z::RotationZ;
use crate::quantum::operator::simple::controlled::Controlled;
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::sized::SizedGate;

pub mod rotation_hadamard;
pub mod rotation_pauli_x;
pub mod rotation_pauli_y;
pub mod rotation_pauli_z;
pub mod rotation_swap;
pub mod rotation_x;
pub mod rotation_y;
pub mod rotation_z;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi, enum_reimport_module))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(tag = "type"))]
#[derive(Copy, Clone, PartialEq)]
pub enum Rotation {
    // Rotations
    RotationX(RotationX),
    RotationY(RotationY),
    RotationZ(RotationZ),
    RotationPauliX(RotationPauliX),
    RotationPauliY(RotationPauliY),
    RotationPauliZ(RotationPauliZ),
    RotationHadamard(RotationHadamard),
    RotationSwap(RotationSwap),
    // Controlled Rotations
    ControlledRotationX(Controlled<2, RotationX>),
    ControlledRotationY(Controlled<2, RotationY>),
    ControlledRotationZ(Controlled<2, RotationZ>),
    ControlledRotationPauliX(Controlled<2, RotationPauliX>),
    ControlledRotationPauliY(Controlled<2, RotationPauliY>),
    ControlledRotationPauliZ(Controlled<2, RotationPauliZ>),
    ControlledRotationHadamard(Controlled<2, RotationHadamard>),
    ControlledRotationSwap(Controlled<3, RotationSwap>),
    // Controlled Controlled Rotations
    ControlledControlledRotationX(Controlled<3, Controlled<2, RotationX>>),
    ControlledControlledRotationY(Controlled<3, Controlled<2, RotationY>>),
    ControlledControlledRotationZ(Controlled<3, Controlled<2, RotationZ>>),
    ControlledControlledRotationPauliX(Controlled<3, Controlled<2, RotationPauliX>>),
    ControlledControlledRotationPauliY(Controlled<3, Controlled<2, RotationPauliY>>),
    ControlledControlledRotationPauliZ(Controlled<3, Controlled<2, RotationPauliZ>>),
    ControlledControlledRotationHadamard(Controlled<3, Controlled<2, RotationHadamard>>),
    ControlledControlledRotationSwap(Controlled<4, Controlled<3, RotationSwap>>),
}

impl Rotation {
    pub fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        match self {
            Rotation::RotationX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationPauliX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationPauliY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationPauliZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationHadamard(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationPauliX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationPauliY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationPauliZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationHadamard(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationPauliX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationPauliY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationPauliZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationHadamard(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RotationSwap(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledRotationSwap(gate) => gate.apply_parameterized(theta, computer),
            Rotation::ControlledControlledRotationSwap(gate) => gate.apply_parameterized(theta, computer),
        }
    }

    pub fn get_parameterized(&self, theta: f64) -> SizedGate {
        match self {
            Rotation::RotationX(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationY(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationZ(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationPauliX(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationPauliY(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationPauliZ(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationHadamard(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationPauliX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationPauliY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationPauliZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationHadamard(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationPauliX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationPauliY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationPauliZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationHadamard(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RotationSwap(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledRotationSwap(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::ControlledControlledRotationSwap(gate) => SizedGate::G4(gate.parameterized()(gate, theta), gate.wires()),
        }
    }

    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            Rotation::RotationX(gate) => gate.apply(computer),
            Rotation::RotationY(gate) => gate.apply(computer),
            Rotation::RotationZ(gate) => gate.apply(computer),
            Rotation::RotationPauliX(gate) => gate.apply(computer),
            Rotation::RotationPauliY(gate) => gate.apply(computer),
            Rotation::RotationPauliZ(gate) => gate.apply(computer),
            Rotation::RotationHadamard(gate) => gate.apply(computer),
            Rotation::ControlledRotationX(gate) => gate.apply(computer),
            Rotation::ControlledRotationY(gate) => gate.apply(computer),
            Rotation::ControlledRotationZ(gate) => gate.apply(computer),
            Rotation::ControlledRotationPauliX(gate) => gate.apply(computer),
            Rotation::ControlledRotationPauliY(gate) => gate.apply(computer),
            Rotation::ControlledRotationPauliZ(gate) => gate.apply(computer),
            Rotation::ControlledRotationHadamard(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationX(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationY(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationZ(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationPauliX(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationPauliY(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationPauliZ(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationHadamard(gate) => gate.apply(computer),
            Rotation::RotationSwap(gate) => gate.apply(computer),
            Rotation::ControlledRotationSwap(gate) => gate.apply(computer),
            Rotation::ControlledControlledRotationSwap(gate) => gate.apply(computer),
        }
    }
}