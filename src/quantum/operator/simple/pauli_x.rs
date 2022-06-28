use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::operator::rotation::rotation_pauli_x::RotationPauliX;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::computer::QuantumComputer;

pub static PAULI_X: Gate<1> = Gate::new([
    [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

/// The Pauli-X Gate @see https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli_gates_(X,Y,Z)
#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct PauliX {
    wire: usize,
}

impl PauliX {
    pub fn new(wire: usize) -> PauliX {
        PauliX { wire }
    }
}

impl ToGate<1> for PauliX {
    fn to_gate(&self) -> Gate<1> {
        PAULI_X
    }
}

impl ApplyGate<1> for PauliX {
    fn apply(&self, computer: &mut QuantumComputer) {
        PAULI_X.apply(computer, self.wires());
    }
}

impl UsedWires<1> for PauliX {
    fn wires(&self) -> [usize; 1] {
        [self.wire]
    }
}

impl Parameterized<1> for PauliX {
    fn parameterized(&self) -> fn(&PauliX, f64) -> Gate<1> {
        fn create_parameterized(g: &PauliX, theta: f64) -> Gate<1> {
            RotationPauliX::new(g.wire, theta).to_gate()
        }

        create_parameterized
    }
}
