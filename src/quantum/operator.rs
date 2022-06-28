use crate::quantum::operator::rotation::Rotation;
use crate::quantum::operator::simple::Simple;
use crate::quantum::operator::special::Special;
use crate::quantum::computer::QuantumComputer;

pub mod simple;
pub mod special;
pub mod rotation;
pub mod traits;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(untagged))]
#[derive(Copy, Clone, PartialEq)]
pub enum OperatorType {
    Simple(Simple),
    Rotation(Rotation),
    Special(Special),
}

impl OperatorType {
    pub fn apply_parameterized(&self, theta: f64, computer: &mut QuantumComputer) {
        match self {
            OperatorType::Simple(simple) => simple.apply_parameterized(theta, computer),
            OperatorType::Rotation(rotation) => rotation.apply_parameterized(theta, computer),
            OperatorType::Special(special) => special.apply(computer),
        }
    }

    pub fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            OperatorType::Simple(simple) => simple.apply(computer),
            OperatorType::Rotation(rotation) => rotation.apply(computer),
            OperatorType::Special(special) => special.apply(computer),
        }
    }
}
