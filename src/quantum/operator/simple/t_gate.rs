use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::operator::rotation::rotation_pauli_z::RotationPauliZ;
use crate::quantum::computer::QuantumComputer;
use core::f64::consts::SQRT_2;

pub static T_GATE: Gate<1> = Gate::new([
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    [
        Complex::new(0.0, 0.0),
        Complex::new(1.0 / SQRT_2, 1.0 / SQRT_2),
    ],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct TGate {
    wire: u32,
}

impl TGate {
    pub fn new(wire: u32) -> TGate {
        TGate { wire }
    }
}

impl ToGate<1> for TGate {
    fn to_gate(&self) -> Gate<1> {
        T_GATE
    }
}

impl UsedWires<1> for TGate {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for TGate {
    fn apply(&self, computer: &mut QuantumComputer) {
        T_GATE.apply(computer, self.wires());
    }
}

impl Parameterized<1> for TGate {
    fn parameterized(&self) -> fn(&TGate, f64) -> Gate<1> {
        fn create_parameterized(g: &TGate, phi: f64) -> Gate<1> {
            RotationPauliZ::new(g.wire, phi / 4.0).to_gate()
        }

        create_parameterized
    }
}
