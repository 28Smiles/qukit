use crate::runtime::ket::Ket;
use crate::runtime::non_unitary::NonUnitaryOperators;
use crate::runtime::register::Register;
use crate::runtime::unitary::{UnitaryOperator, UnitaryOperators};

pub(crate) enum Operator {
    Unitary(UnitaryOperators),
    NonUnitary(NonUnitaryOperators),
}

impl UnitaryOperator for Operator {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        match self {
            Operator::Unitary(u) => u.apply(ket, register),
            Operator::NonUnitary(nu) => nu.apply(ket, register),
        }
    }
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for Operator {
            fn from(value: $type) -> Self {
                Operator::$name(value)
            }
        }
    };
}

impl_from_trait!(Unitary, UnitaryOperators);
impl_from_trait!(NonUnitary, NonUnitaryOperators);
