use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_pauli_y::RotationPauliY;
use crate::quantum::computer::QuantumComputer;

pub static PAULI_Y: Gate<1> = Gate::new([
    [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
    [Complex::new(0.0, -1.0), Complex::new(0.0, 0.0)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct PauliY {
    wire: u32,
}

impl PauliY {
    pub fn new(wire: u32) -> PauliY {
        PauliY { wire }
    }
}

impl ToGate<1> for PauliY {
    fn to_gate(&self) -> Gate<1> {
        PAULI_Y
    }
}

impl ApplyGate<1> for PauliY {
    fn apply(&self, computer: &mut QuantumComputer) {
        PAULI_Y.apply(computer, self.wires());
    }
}

impl UsedWires<1> for PauliY {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}

impl Parameterized<1> for PauliY {
    fn parameterized(&self) -> fn(&PauliY, f64) -> Gate<1> {
        fn create_parameterized(g: &PauliY, theta: f64) -> Gate<1> {
            RotationPauliY::new(g.wire, theta).to_gate()
        }

        create_parameterized
    }
}
