use crate::runtime::dynamic_sized::unitary_operator::DynamicSizedUnitaryOperator;
use crate::runtime::non_unitary::measurement::Measurement;
use crate::runtime::non_unitary::reset::Reset;
use crate::toolbox::controlled::Controlled;
use crate::toolbox::operator::Operator;
use crate::toolbox::rotation::Rotation;

pub(crate)mod operator;
pub(crate)mod rotation;
pub(crate)mod parameterized;
pub(crate)mod controlled;

#[derive(Clone)]
pub(crate)enum Tool {
    Operator(Operator),
    Rotation(Rotation),
    Controlled(Controlled),
    Measurement(Measurement),
    Reset(Reset),

    Custom(DynamicSizedUnitaryOperator),
    None,
}

impl const Default for Tool {
    fn default() -> Self {
        Tool::None
    }
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for Tool {
            fn from(value: $type) -> Self {
                Tool::$name(value)
            }
        }
    };
}

impl_from_trait!(Operator, Operator);
impl_from_trait!(Rotation, Rotation);
impl_from_trait!(Controlled, Controlled);
impl_from_trait!(Measurement, Measurement);
impl_from_trait!(Reset, Reset);
