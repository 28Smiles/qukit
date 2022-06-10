use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_hadamard::RotationHadamard;
use core::f64::consts::SQRT_2;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

pub static HADAMARD: Gate<1> = Gate::new([
    [
        Complex::new(1.0 / SQRT_2, 0.0),
        Complex::new(1.0 / SQRT_2, 0.0),
    ],
    [
        Complex::new(1.0 / SQRT_2, 0.0),
        Complex::new(-1.0 / SQRT_2, 0.0),
    ],
]);

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct Hadamard {
    wire: u32,
}

impl Hadamard {
    pub fn new(wire: u32) -> Hadamard {
        Hadamard { wire }
    }
}

impl ToGate<1> for Hadamard {
    fn to_gate(&self) -> Gate<1> {
        HADAMARD
    }
}

impl UsedWires<1> for Hadamard {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for Hadamard {
    fn apply(&self, computer: &mut QuantumComputer) {
        HADAMARD.apply(computer, self.wires());
    }
}

impl Parameterized<1> for Hadamard {
    fn parameterized(&self) -> fn(&Hadamard, f64) -> Gate<1> {
        fn create_parameterized(g: &Hadamard, theta: f64) -> Gate<1> {
            RotationHadamard::new(g.wire, theta).to_gate()
        }

        create_parameterized
    }
}
