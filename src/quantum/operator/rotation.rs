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
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "wasm-pack", serde(tag = "type"))]
#[derive(Copy, Clone, PartialEq)]
pub enum Rotation {
    // Rotations
    RX(RotationX),
    RY(RotationY),
    RZ(RotationZ),
    RPX(RotationPauliX),
    RPY(RotationPauliY),
    RPZ(RotationPauliZ),
    RH(RotationHadamard),
    RSWAP(RotationSwap),
    // Controlled Rotations
    CRX(Controlled<2, RotationX>),
    CRY(Controlled<2, RotationY>),
    CRZ(Controlled<2, RotationZ>),
    CRPX(Controlled<2, RotationPauliX>),
    CRPY(Controlled<2, RotationPauliY>),
    CRPZ(Controlled<2, RotationPauliZ>),
    CRH(Controlled<2, RotationHadamard>),
    CRSWAP(Controlled<3, RotationSwap>),
    // Controlled Controlled Rotations
    CCRX(Controlled<3, Controlled<2, RotationX>>),
    CCRY(Controlled<3, Controlled<2, RotationY>>),
    CCRZ(Controlled<3, Controlled<2, RotationZ>>),
    CCRPX(Controlled<3, Controlled<2, RotationPauliX>>),
    CCRPY(Controlled<3, Controlled<2, RotationPauliY>>),
    CCRPZ(Controlled<3, Controlled<2, RotationPauliZ>>),
    CCRH(Controlled<3, Controlled<2, RotationHadamard>>),
    CCRSWAP(Controlled<4, Controlled<3, RotationSwap>>),
}

impl Rotation {
    pub fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        match self {
            Rotation::RX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RPX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RPY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RPZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RH(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRPX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRPY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRPZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRH(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRPX(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRPY(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRPZ(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRH(gate) => gate.apply_parameterized(theta, computer),
            Rotation::RSWAP(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CRSWAP(gate) => gate.apply_parameterized(theta, computer),
            Rotation::CCRSWAP(gate) => gate.apply_parameterized(theta, computer),
        }
    }

    pub fn get_parameterized(&self, theta: f64) -> SizedGate {
        match self {
            Rotation::RX(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RY(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RZ(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RPX(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RPY(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RPZ(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RH(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRPX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRPY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRPZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRH(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRPX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRPY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRPZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRH(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::RSWAP(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CRSWAP(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Rotation::CCRSWAP(gate) => SizedGate::G4(gate.parameterized()(gate, theta), gate.wires()),
        }
    }

    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            Rotation::RX(gate) => gate.apply(computer),
            Rotation::RY(gate) => gate.apply(computer),
            Rotation::RZ(gate) => gate.apply(computer),
            Rotation::RPX(gate) => gate.apply(computer),
            Rotation::RPY(gate) => gate.apply(computer),
            Rotation::RPZ(gate) => gate.apply(computer),
            Rotation::RH(gate) => gate.apply(computer),
            Rotation::CRX(gate) => gate.apply(computer),
            Rotation::CRY(gate) => gate.apply(computer),
            Rotation::CRZ(gate) => gate.apply(computer),
            Rotation::CRPX(gate) => gate.apply(computer),
            Rotation::CRPY(gate) => gate.apply(computer),
            Rotation::CRPZ(gate) => gate.apply(computer),
            Rotation::CRH(gate) => gate.apply(computer),
            Rotation::CCRX(gate) => gate.apply(computer),
            Rotation::CCRY(gate) => gate.apply(computer),
            Rotation::CCRZ(gate) => gate.apply(computer),
            Rotation::CCRPX(gate) => gate.apply(computer),
            Rotation::CCRPY(gate) => gate.apply(computer),
            Rotation::CCRPZ(gate) => gate.apply(computer),
            Rotation::CCRH(gate) => gate.apply(computer),
            Rotation::RSWAP(gate) => gate.apply(computer),
            Rotation::CRSWAP(gate) => gate.apply(computer),
            Rotation::CCRSWAP(gate) => gate.apply(computer),
        }
    }
}