use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_pauli_z::RotationPauliZ;
use crate::quantum::computer::QuantumComputer;

pub static S_GATE: Gate<1> = Gate::new([
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct SGate {
    wire: u32,
}

impl SGate {
    pub fn new(wire: u32) -> SGate {
        SGate { wire }
    }
}

impl ToGate<1> for SGate {
    fn to_gate(&self) -> Gate<1> {
        S_GATE
    }
}

impl UsedWires<1> for SGate {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for SGate {
    fn apply(&self, computer: &mut QuantumComputer) {
        S_GATE.apply(computer, self.wires());
    }
}

impl Parameterized<1> for SGate {
    fn parameterized(&self) -> fn(&SGate, f64) -> Gate<1> {
        fn create_parameterized(g: &SGate, phi: f64) -> Gate<1> {
            RotationPauliZ::new(g.wire, phi / 2.0).to_gate()
        }

        create_parameterized
    }
}
