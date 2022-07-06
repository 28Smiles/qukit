pub(crate) mod measurement;
pub(crate) mod reset;

use crate::runtime::ket::Ket;
use crate::runtime::operator::Operator;
use crate::runtime::non_unitary::measurement::Measurement;
use crate::runtime::non_unitary::reset::Reset;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;

pub(crate) enum NonUnitaryOperators {
    Measurement(Measurement),
    Reset(Reset),
}

impl UnitaryOperator for NonUnitaryOperators {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        match self {
            NonUnitaryOperators::Measurement(nu) => nu.apply(ket, register),
            NonUnitaryOperators::Reset(nu) => nu.apply(ket, register),
        }
    }
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for NonUnitaryOperators {
            fn from(value: $type) -> Self {
                NonUnitaryOperators::$name(value)
            }
        }
        impl From<$type> for Operator {
            fn from(value: $type) -> Self {
                Operator::NonUnitary(NonUnitaryOperators::$name(value))
            }
        }
    };
}

impl_from_trait!(Measurement, Measurement);
impl_from_trait!(Reset, Reset);
