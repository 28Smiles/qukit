use crate::complex::Complex;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate, UsedWires};
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::rotation::rotation_pauli_z::RotationPauliZ;
use crate::quantum::computer::QuantumComputer;

pub static PAULI_Z: Gate<1> = Gate::new([
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
]);

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct PauliZ {
    wire: usize,
}

impl PauliZ {
    pub fn new(wire: usize) -> PauliZ {
        PauliZ { wire }
    }
}

impl ToGate<1> for PauliZ {
    fn to_gate(&self) -> Gate<1> {
        PAULI_Z
    }
}

impl UsedWires<1> for PauliZ {
    fn wires(&self) -> [usize; 1] {
        [self.wire]
    }
}

impl ApplyGate<1> for PauliZ {
    fn apply(&self, computer: &mut QuantumComputer) {
        PAULI_Z.apply(computer, self.wires());
    }
}

impl Parameterized<1> for PauliZ {
    fn parameterized(&self) -> fn(&PauliZ, f64) -> Gate<1> {
        fn create_parameterized(g: &PauliZ, phi: f64) -> Gate<1> {
            RotationPauliZ::new(g.wire, phi).to_gate()
        }

        create_parameterized
    }
}
