use crate::quantum::operator::special::measurement::Measurement;
use crate::quantum::operator::special::reset::Reset;
use crate::quantum::operator::traits::ApplyGate;
use crate::quantum::computer::QuantumComputer;

pub mod measurement;
pub mod reset;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(tag = "type"))]
#[derive(Copy, Clone, PartialEq)]
pub enum Special {
    Measurement(Measurement),
    Reset(Reset),
}

impl Special {
    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            Special::Measurement(gate) => gate.apply(computer),
            Special::Reset(gate) => gate.apply(computer),
        }
    }
}
