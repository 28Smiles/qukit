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
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "wasm-pack", serde(tag = "type"))]
#[derive(Copy, Clone, PartialEq)]
pub enum Simple {
    // Primitives
    X(PauliX),
    Y(PauliY),
    Z(PauliZ),
    H(Hadamard),
    T(TGate),
    S(SGate),
    SINV(SGateInverse),
    XR(PauliXRoot),
    SWAP(Swap),
    SWAPROOT(SwapRoot),
    // Controlled
    CX(Controlled<2, PauliX>),
    CY(Controlled<2, PauliY>),
    CZ(Controlled<2, PauliZ>),
    CH(Controlled<2, Hadamard>),
    CT(Controlled<2, TGate>),
    CS(Controlled<2, SGate>),
    CSINV(Controlled<2, SGateInverse>),
    CXR(Controlled<2, PauliXRoot>),
    CSWAP(Controlled<3, Swap>),
    CSWAPROOT(Controlled<3, SwapRoot>),
    // Controlled Controlled
    CCPX(Controlled<3, Controlled<2, PauliX>>),
    CCPY(Controlled<3, Controlled<2, PauliY>>),
    CCPZ(Controlled<3, Controlled<2, PauliZ>>),
    CCH(Controlled<3, Controlled<2, Hadamard>>),
    CCT(Controlled<3, Controlled<2, TGate>>),
    CCS(Controlled<3, Controlled<2, SGate>>),
    CCSINV(Controlled<3, Controlled<2, SGateInverse>>),
    CCXR(Controlled<3, Controlled<2, PauliXRoot>>),
}

impl Simple {
    pub fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        match self {
            Simple::X(gate) => gate.apply_parameterized(theta, computer),
            Simple::Y(gate) => gate.apply_parameterized(theta, computer),
            Simple::Z(gate) => gate.apply_parameterized(theta, computer),
            Simple::H(gate) => gate.apply_parameterized(theta, computer),
            Simple::T(gate) => gate.apply_parameterized(theta, computer),
            Simple::S(gate) => gate.apply_parameterized(theta, computer),
            Simple::SINV(gate) => gate.apply_parameterized(theta, computer),
            Simple::XR(gate) => gate.apply_parameterized(theta, computer),
            Simple::CX(gate) => gate.apply_parameterized(theta, computer),
            Simple::CY(gate) => gate.apply_parameterized(theta, computer),
            Simple::CZ(gate) => gate.apply_parameterized(theta, computer),
            Simple::CH(gate) => gate.apply_parameterized(theta, computer),
            Simple::CT(gate) => gate.apply_parameterized(theta, computer),
            Simple::CS(gate) => gate.apply_parameterized(theta, computer),
            Simple::CSINV(gate) => gate.apply_parameterized(theta, computer),
            Simple::CXR(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCPX(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCPY(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCPZ(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCH(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCT(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCS(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCSINV(gate) => gate.apply_parameterized(theta, computer),
            Simple::CCXR(gate) => gate.apply_parameterized(theta, computer),
            Simple::SWAP(gate) => gate.apply_parameterized(theta, computer),
            Simple::CSWAP(gate) => gate.apply_parameterized(theta, computer),
            Simple::SWAPROOT(gate) => gate.apply_parameterized(theta, computer),
            Simple::CSWAPROOT(gate) => gate.apply_parameterized(theta, computer),
        }
    }

    pub fn get_parameterized(&self, theta: f64) -> SizedGate {
        match self {
            Simple::X(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::Y(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::Z(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::H(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::T(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::S(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SINV(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::XR(gate) => SizedGate::G1(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CX(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CY(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CZ(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CH(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CT(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CS(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CSINV(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CXR(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCPX(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCPY(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCPZ(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCH(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCT(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCS(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCSINV(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CCXR(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SWAP(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CSWAP(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
            Simple::SWAPROOT(gate) => SizedGate::G2(gate.parameterized()(gate, theta), gate.wires()),
            Simple::CSWAPROOT(gate) => SizedGate::G3(gate.parameterized()(gate, theta), gate.wires()),
        }
    }

    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            Simple::X(gate) => gate.apply(computer),
            Simple::Y(gate) => gate.apply(computer),
            Simple::Z(gate) => gate.apply(computer),
            Simple::H(gate) => gate.apply(computer),
            Simple::T(gate) => gate.apply(computer),
            Simple::S(gate) => gate.apply(computer),
            Simple::SINV(gate) => gate.apply(computer),
            Simple::XR(gate) => gate.apply(computer),
            Simple::CX(gate) => gate.apply(computer),
            Simple::CY(gate) => gate.apply(computer),
            Simple::CZ(gate) => gate.apply(computer),
            Simple::CH(gate) => gate.apply(computer),
            Simple::CT(gate) => gate.apply(computer),
            Simple::CS(gate) => gate.apply(computer),
            Simple::CSINV(gate) => gate.apply(computer),
            Simple::CXR(gate) => gate.apply(computer),
            Simple::CCPX(gate) => gate.apply(computer),
            Simple::CCPY(gate) => gate.apply(computer),
            Simple::CCPZ(gate) => gate.apply(computer),
            Simple::CCH(gate) => gate.apply(computer),
            Simple::CCT(gate) => gate.apply(computer),
            Simple::CCS(gate) => gate.apply(computer),
            Simple::CCSINV(gate) => gate.apply(computer),
            Simple::CCXR(gate) => gate.apply(computer),
            Simple::SWAP(gate) => gate.apply(computer),
            Simple::CSWAP(gate) => gate.apply(computer),
            Simple::SWAPROOT(gate) => gate.apply(computer),
            Simple::CSWAPROOT(gate) => gate.apply(computer),
        }
    }
}