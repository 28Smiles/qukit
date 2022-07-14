use crate::toolbox::rotation::hadamard::RotationHadamard;
use crate::toolbox::rotation::pauli_x::RotationPauliX;
use crate::toolbox::rotation::pauli_y::RotationPauliY;
use crate::toolbox::rotation::pauli_z::RotationPauliZ;
use crate::toolbox::rotation::swap::RotationSwap;
use crate::toolbox::rotation::x::RotationX;
use crate::toolbox::rotation::y::RotationY;
use crate::toolbox::rotation::z::RotationZ;
use crate::toolbox::Tool;

pub(crate)mod x;
pub(crate)mod y;
pub(crate)mod z;
pub(crate)mod pauli_x;
pub(crate)mod pauli_y;
pub(crate)mod pauli_z;
pub(crate)mod hadamard;
pub(crate)mod swap;

#[derive(Copy, Clone, Debug)]
pub(crate)enum Rotation {
    Hadamard(RotationHadamard),
    X(RotationX),
    Y(RotationY),
    Z(RotationZ),
    PauliX(RotationPauliX),
    PauliY(RotationPauliY),
    PauliZ(RotationPauliZ),
    Swap(RotationSwap),
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for Rotation {
            fn from(value: $type) -> Self {
                Rotation::$name(value)
            }
        }
        impl From<$type> for Tool {
            fn from(value: $type) -> Self {
                Tool::Rotation(Rotation::$name(value))
            }
        }
    };
}

impl_from_trait!(Hadamard, RotationHadamard);
impl_from_trait!(X, RotationX);
impl_from_trait!(Y, RotationY);
impl_from_trait!(Z, RotationZ);
impl_from_trait!(PauliX, RotationPauliX);
impl_from_trait!(PauliY, RotationPauliY);
impl_from_trait!(PauliZ, RotationPauliZ);
impl_from_trait!(Swap, RotationSwap);

pub(crate) mod rotation_macro {
    #[macro_export]
    macro_rules! impl_rotation {
        ($matrix:ident, $type:ty, $name:ident, 1, $doc: expr) => {
            #[doc = $doc]
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub(crate)struct $name(f64, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>);

            impl $name {
                pub(crate) const fn new(theta: f64, wire: usize) -> $name {
                    $name(theta, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Owned($matrix(theta)),
                        [wire],
                        None,
                    ))
                }

                pub(crate) const fn new_classically_controlled(theta: f64, wire: usize, classical_control: usize) -> $name {
                    $name(theta, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Owned($matrix(theta)),
                        [wire],
                        Some(classical_control),
                    ))
                }

                pub(crate)fn operator(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type> {
                    self.1
                }
            }

            impl const crate::toolbox::parameterized::Parameterized<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>, $name> for $name {
                fn parameterized(&self, theta: f64) -> $name {
                    if let Some(classical_control) = self.classical_control() {
                        <$name>::new_classically_controlled(self.0 / core::f64::consts::PI * theta, self.wires()[0], classical_control)
                    } else {
                        <$name>::new(theta, self.wires()[0])
                    }
                }
            }

            impl const core::ops::Deref for $name {
                type Target = crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>;

                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }

            impl const Into<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>> for $name {
                fn into(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type> {
                    self.1
                }
            }
        };

        ($matrix:ident, $type:ty, $name:ident, 2, $doc: expr) => {
            #[doc = $doc]
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub(crate)struct $name(f64, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>);

            impl $name {
                pub(crate) const fn new(theta: f64, wire_0: usize, wire_1: usize) -> $name {
                    $name(theta, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Owned($matrix(theta)),
                        [wire_0, wire_1],
                        None,
                    ))
                }

                pub(crate) const fn new_classically_controlled(theta: f64, wire_0: usize, wire_1: usize, classical_control: usize) -> $name {
                    $name(theta, crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Owned($matrix(theta)),
                        [wire_0, wire_1],
                        Some(classical_control),
                    ))
                }

                pub(crate)fn operator(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type> {
                    self.1
                }
            }

            impl const crate::toolbox::parameterized::Parameterized<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>, $name> for $name {
                fn parameterized(&self, theta: f64) -> $name {
                    if let Some(classical_control) = self.classical_control() {
                        <$name>::new_classically_controlled(self.0 / core::f64::consts::PI * theta, self.wires()[0], self.wires()[1], classical_control)
                    } else {
                        <$name>::new(theta, self.wires()[0], self.wires()[1])
                    }
                }
            }

            impl const core::ops::Deref for $name {
                type Target = crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>;

                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }

            impl const Into<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>> for $name {
                fn into(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type> {
                    self.1
                }
            }
        };
    }
}
