use core::f64::consts::PI;
use libm::{cos, sin};

use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

pub fn rz(phi: f64) -> Gate<1> {
    Gate::new([
        [
            Complex::new(cos(phi / 2.0), -sin(phi / 2.0)),
            Complex::new(0.0, 0.0),
        ],
        [
            Complex::new(0.0, 0.0),
            Complex::new(cos(phi / 2.0), sin(phi / 2.0)),
        ],
    ])
}

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct RotationZ {
    wire: usize,
    theta: f64,
}

impl RotationZ {
    pub fn new(wire: usize, theta: f64) -> RotationZ {
        RotationZ { wire, theta }
    }
}

impl ToGate<1> for RotationZ {
    fn to_gate(&self) -> Gate<1> {
        rz(self.theta)
    }
}

impl UsedWires<1> for RotationZ {
    fn wires(&self) -> [usize; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationZ {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, [self.wire]);
    }
}

impl Parameterized<1> for RotationZ {
    fn parameterized(&self) -> fn(&RotationZ, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationZ, theta: f64) -> Gate<1> {
            RotationZ::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}
