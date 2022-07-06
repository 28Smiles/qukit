use crate::complex::Complex;
use crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator;
use crate::runtime::dynamic_sized::unitary_operator::DynamicSizedUnitaryOperator;
use crate::runtime::ket::Ket;
use crate::runtime::operator::Operator;
use crate::runtime::register::Register;

pub(crate)trait UnitaryOperator {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket;
}

pub(crate)enum UnitaryOperators {
    D1F(ConstSizedUnitaryOperator<1, f64>),
    D2F(ConstSizedUnitaryOperator<2, f64>),
    D3F(ConstSizedUnitaryOperator<3, f64>),
    D1C(ConstSizedUnitaryOperator<1, Complex>),
    D2C(ConstSizedUnitaryOperator<2, Complex>),
    D3C(ConstSizedUnitaryOperator<3, Complex>),
    D(DynamicSizedUnitaryOperator),
}

impl UnitaryOperator for UnitaryOperators {
    fn apply(&self, ket: Ket, register: &mut Register) -> Ket {
        match self {
            UnitaryOperators::D1F(u) => u.apply(ket, register),
            UnitaryOperators::D2F(u) => u.apply(ket, register),
            UnitaryOperators::D3F(u) => u.apply(ket, register),
            UnitaryOperators::D1C(u) => u.apply(ket, register),
            UnitaryOperators::D2C(u) => u.apply(ket, register),
            UnitaryOperators::D3C(u) => u.apply(ket, register),
            UnitaryOperators::D(u) => u.apply(ket, register),
        }
    }
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for UnitaryOperators {
            fn from(value: $type) -> Self {
                UnitaryOperators::$name(value)
            }
        }
        impl From<$type> for Operator {
            fn from(value: $type) -> Self {
                Operator::Unitary(UnitaryOperators::$name(value))
            }
        }
    };
}

impl_from_trait!(D1F, ConstSizedUnitaryOperator<1, f64>);
impl_from_trait!(D2F, ConstSizedUnitaryOperator<2, f64>);
impl_from_trait!(D3F, ConstSizedUnitaryOperator<3, f64>);
impl_from_trait!(D1C, ConstSizedUnitaryOperator<1, Complex>);
impl_from_trait!(D2C, ConstSizedUnitaryOperator<2, Complex>);
impl_from_trait!(D3C, ConstSizedUnitaryOperator<3, Complex>);
impl_from_trait!(D, DynamicSizedUnitaryOperator);
