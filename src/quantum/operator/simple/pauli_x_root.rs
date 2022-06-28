use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_pauli_x::RotationPauliX;
use crate::quantum::computer::QuantumComputer;

pub static PAULI_X_ROOT: Gate<1> = Gate::new([
    [Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)],
    [Complex::new(0.5, -0.5), Complex::new(0.5, 0.5)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct PauliXRoot {
    wire: usize,
}

impl PauliXRoot {
    pub fn new(wire: usize) -> PauliXRoot {
        PauliXRoot { wire }
    }
}

impl ToGate<1> for PauliXRoot {
    fn to_gate(&self) -> Gate<1> {
        PAULI_X_ROOT
    }
}

impl ApplyGate<1> for PauliXRoot {
    fn apply(&self, computer: &mut QuantumComputer) {
        PAULI_X_ROOT.apply(computer, self.wires());
    }
}

impl UsedWires<1> for PauliXRoot {
    fn wires(&self) -> [usize; 1] {
        [self.wire]
    }
}

impl Parameterized<1> for PauliXRoot {
    fn parameterized(&self) -> fn(&PauliXRoot, f64) -> Gate<1> {
        fn create_parameterized(g: &PauliXRoot, theta: f64) -> Gate<1> {
            RotationPauliX::new(g.wire, theta / 2.0).to_gate()
        }

        create_parameterized
    }
}
