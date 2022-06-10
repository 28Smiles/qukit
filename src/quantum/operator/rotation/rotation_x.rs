use core::f64::consts::PI;
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::computer::QuantumComputer;
use libm::{cos, sin};

pub fn rx(theta: f64) -> Gate<1> {
    Gate::new([
        [
            Complex::new(cos(theta / 2.0), 0.0),
            Complex::new(0.0, -sin(theta / 2.0)),
        ],
        [
            Complex::new(0.0, -sin(theta / 2.0)),
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
pub struct RotationX {
    wire: u32,
    theta: f64,
}

impl RotationX {
    pub fn new(wire: u32, theta: f64) -> RotationX {
        RotationX { wire, theta }
    }
}

impl ToGate<1> for RotationX {
    fn to_gate(&self) -> Gate<1> {
        rx(self.theta)
    }
}

impl UsedWires<1> for RotationX {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for RotationX {
    fn apply(&self, computer: &mut QuantumComputer) {
        self.to_gate().apply(computer, [self.wire]);
    }
}

impl Parameterized<1> for RotationX {
    fn parameterized(&self) -> fn(&RotationX, f64) -> Gate<1> {
        fn create_parameterized(g: &RotationX, theta: f64) -> Gate<1> {
            RotationX::new(g.wire, g.theta / PI * theta).to_gate()
        }

        create_parameterized
    }
}
