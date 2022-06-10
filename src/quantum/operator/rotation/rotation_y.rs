use core::f64::consts::PI;
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;
use libm::{cos, sin};

pub fn ry(theta: f64) -> Gate<1> {
    Gate::new([
        [
            Complex::new(cos(theta / 2.0), 0.0),
            Complex::new(sin(theta / 2.0), 0.0),
        ],
        [
            Complex::new(-sin(theta / 2.0), 0.0),
            Complex::new(cos(theta / 2.0), 0.0),
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
pub struct RotationY {
    wire: u32,
    theta: f64,
}

impl RotationY {
    pub fn new(wire: u32, theta: f64) -> RotationY {
        RotationY { wire, theta }
    }
}

impl ToGate<1> for RotationY {
    fn to_gate(&self) -> Gate<1> {
        ry(self.theta)
    }
}

impl UsedWires<1> for RotationY {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationY {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, [self.wire]);
    }
}

impl Parameterized<1> for RotationY {
    fn parameterized(&self) -> fn(&RotationY, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationY, theta: f64) -> Gate<1> {
            RotationY::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}
