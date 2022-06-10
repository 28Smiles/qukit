use libm::sqrt;
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::operator::traits::{ApplyGate, UsedWires};
use crate::quantum::computer::QuantumComputer;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
#[derive(Copy, Clone, PartialEq)]
pub struct Reset {
    wire: u32,
    state: bool,
}

impl Reset {
    pub fn new(wire: u32, state: bool) -> Reset {
        Reset {
            wire,
            state,
        }
    }
}

impl ApplyGate<1> for Reset {
    fn apply(&self, computer: &mut QuantumComputer) {
        let proballily = computer.probability(self.wire);

        if self.state {
            Gate::new([
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(sqrt(1.0 / proballily), 0.0)],
            ])
        } else {
            Gate::new([
                [Complex::new(sqrt(1.0 / proballily), 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
            ])
        }.apply(computer, self.wires())
    }
}

impl UsedWires<1> for Reset {
    fn wires(&self) -> [u32; 1] {
        [self.wire]
    }
}
