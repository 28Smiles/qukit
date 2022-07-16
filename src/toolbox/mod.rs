use crate::runtime::dynamic_sized::unitary_operator::DynamicSizedUnitaryOperator;
use crate::runtime::ket::Ket;
use crate::runtime::non_unitary::measurement::Measurement;
use crate::runtime::non_unitary::reset::Reset;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;
use crate::toolbox::controlled::Controlled;
use crate::toolbox::operator::Operator;
use crate::toolbox::rotation::Rotation;

pub(crate)mod operator;
pub(crate)mod rotation;
pub(crate)mod parameterized;
pub(crate)mod controlled;

#[derive(Clone, Debug)]
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

impl UnitaryOperator for Tool {
    fn apply(&self, ket: Ket, reg: &mut Register) -> Ket {
        match self {
            Tool::Operator(o) => {
                match o {
                    Operator::Hadamard(o) => o.operator().apply(ket, reg),
                    Operator::PauliX(o) => o.operator().apply(ket, reg),
                    Operator::PauliXRoot(o) => o.operator().apply(ket, reg),
                    Operator::PauliY(o) => o.operator().apply(ket, reg),
                    Operator::PauliZ(o) => o.operator().apply(ket, reg),
                    Operator::Phase(o) => o.operator().apply(ket, reg),
                    Operator::PhaseDagger(o) => o.operator().apply(ket, reg),
                    Operator::PhaseRoot(o) => o.operator().apply(ket, reg),
                    Operator::PhaseRootDagger(o) => o.operator().apply(ket, reg),
                    Operator::Swap(o) => o.operator().apply(ket, reg),
                    Operator::SwapRoot(o) => o.operator().apply(ket, reg),
                }
            }
            Tool::Rotation(o) => {
                match o {
                    Rotation::Hadamard(o) => o.operator().apply(ket, reg),
                    Rotation::X(o) => o.operator().apply(ket, reg),
                    Rotation::Y(o) => o.operator().apply(ket, reg),
                    Rotation::Z(o) => o.operator().apply(ket, reg),
                    Rotation::PauliX(o) => o.operator().apply(ket, reg),
                    Rotation::PauliY(o) => o.operator().apply(ket, reg),
                    Rotation::PauliZ(o) => o.operator().apply(ket, reg),
                    Rotation::Swap(o) => o.operator().apply(ket, reg),
                    Rotation::U(o) => o.operator().apply(ket, reg),
                }
            }
            Tool::Controlled(o) => {
                match o {
                    Controlled::ControlledHadamard(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPauliX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPauliXRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPauliY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPauliZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPhase(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPhaseDagger(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPhaseRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledPhaseRootDagger(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledSwap(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledSwapRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationHadamard(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationPauliX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationPauliY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationPauliZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationSwap(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledRotationU(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledHadamard(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPauliX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPauliXRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPauliY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPauliZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPhase(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPhaseDagger(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPhaseRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledPhaseRootDagger(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledSwap(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledSwapRoot(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationHadamard(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationPauliX(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationPauliY(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationPauliZ(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationSwap(o) => o.operator().apply(ket, reg),
                    Controlled::ControlledControlledRotationU(o) => o.operator().apply(ket, reg),
                }
            }
            Tool::Measurement(o) => o.apply(ket, reg),
            Tool::Reset(o) => o.apply(ket, reg),
            Tool::Custom(o) => o.apply(ket, reg),
            Tool::None => ket,
        }
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
