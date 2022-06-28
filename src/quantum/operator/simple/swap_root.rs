use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::operator::rotation::rotation_swap::RotationSwap;
use crate::quantum::computer::QuantumComputer;

pub static SWAP_ROOT: Gate<2> = Gate::new([
    [
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(0.5, 0.5),
        Complex::new(0.5, -0.5),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(0.5, -0.5),
        Complex::new(0.5, 0.5),
        Complex::new(0.0, 0.0),
    ],
    [
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
    ],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct SwapRoot {
    wire_0: usize,
    wire_1: usize,
}

impl SwapRoot {
    pub fn new(wire_0: usize, wire_1: usize) -> SwapRoot {
        SwapRoot { wire_0, wire_1 }
    }
}

impl ToGate<2> for SwapRoot {
    fn to_gate(&self) -> Gate<2> {
        SWAP_ROOT
    }
}

impl ApplyGate<2> for SwapRoot {
    fn apply(&self, computer: &mut QuantumComputer) {
        SWAP_ROOT.apply(computer, self.wires());
    }
}

impl UsedWires<2> for SwapRoot {
    fn wires(&self) -> [usize; 2] {
        [self.wire_1, self.wire_0]
    }
}

impl Parameterized<2> for SwapRoot {
    fn parameterized(&self) -> fn(&SwapRoot, f64) -> Gate<2> {
        fn create_parameterized(g: &SwapRoot, theta: f64) -> Gate<2> {
            RotationSwap::new(g.wire_0, g.wire_1, theta / 2.0).to_gate()
        }

        create_parameterized
    }
}
