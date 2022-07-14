use crate::toolbox::operator::hadamard::Hadamard;
use crate::toolbox::operator::pauli_x::PauliX;
use crate::toolbox::operator::pauli_x_root::PauliXRoot;
use crate::toolbox::operator::pauli_y::PauliY;
use crate::toolbox::operator::pauli_z::PauliZ;
use crate::toolbox::operator::phase::{Phase, PhaseDagger};
use crate::toolbox::operator::phase_root::{PhaseRoot, PhaseRootDagger};
use crate::toolbox::operator::swap::Swap;
use crate::toolbox::operator::swap_root::SwapRoot;
use crate::toolbox::Tool;

pub(crate)mod hadamard;
pub(crate)mod pauli_x;
pub(crate)mod pauli_y;
pub(crate)mod pauli_z;
pub(crate)mod controlled;
pub(crate)mod pauli_x_root;
pub(crate)mod phase;
pub(crate)mod swap;
pub(crate)mod swap_root;
pub(crate)mod phase_root;
pub(crate)mod c;

#[derive(Copy, Clone, Debug)]
pub(crate)enum Operator {
    Hadamard(Hadamard),
    PauliX(PauliX),
    PauliXRoot(PauliXRoot),
    PauliY(PauliY),
    PauliZ(PauliZ),
    Phase(Phase),
    PhaseDagger(PhaseDagger),
    PhaseRoot(PhaseRoot),
    PhaseRootDagger(PhaseRootDagger),
    Swap(Swap),
    SwapRoot(SwapRoot),
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for Operator {
            fn from(value: $type) -> Self {
                Operator::$name(value)
            }
        }
        impl From<$type> for Tool {
            fn from(value: $type) -> Self {
                Tool::Operator(Operator::$name(value))
            }
        }
    };
}

impl_from_trait!(Hadamard, Hadamard);
impl_from_trait!(PauliX, PauliX);
impl_from_trait!(PauliXRoot, PauliXRoot);
impl_from_trait!(PauliY, PauliY);
impl_from_trait!(PauliZ, PauliZ);
impl_from_trait!(Phase, Phase);
impl_from_trait!(PhaseDagger, PhaseDagger);
impl_from_trait!(PhaseRoot, PhaseRoot);
impl_from_trait!(PhaseRootDagger, PhaseRootDagger);
impl_from_trait!(Swap, Swap);
impl_from_trait!(SwapRoot, SwapRoot);

pub(crate) mod operator_macro {
    #[macro_export]
    macro_rules! impl_operator {
        ($matrix:ident, $type:ty, $type_r:ty, $name:ident, $rotation_type:ty, 1, $theta:expr, $doc: expr) => {
            #[doc = $doc]
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub(crate)struct $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>);

            impl $name {
                pub(crate) const fn new(wire: usize) -> $name {
                    $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Borrowed(&$matrix),
                        [wire],
                        None,
                    ))
                }

                pub(crate) const fn new_classically_controlled(wire: usize, classical_control: usize) -> $name {
                    $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Borrowed(&$matrix),
                        [wire],
                        Some(classical_control),
                    ))
                }

                pub(crate) const fn new_parameterized(theta: f64, wire: usize, classical_control: Option<usize>) -> $rotation_type {
                    if let Some(classical_control) = classical_control {
                        <$rotation_type>::new_classically_controlled(theta, wire, classical_control)
                    } else {
                        <$rotation_type>::new(theta, wire)
                    }
                }

                pub(crate)fn operator(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type> {
                    self.0
                }
            }

            impl const crate::toolbox::parameterized::Parameterized<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type_r>, $rotation_type> for $name {
                fn parameterized(&self, theta: f64) -> $rotation_type {
                    if let Some(classical_control) = self.classical_control() {
                        <$rotation_type>::new_classically_controlled(theta / $theta, self.wires()[0], classical_control)
                    } else {
                        <$rotation_type>::new(theta / $theta, self.wires()[0])
                    }
                }
            }

            impl const core::ops::Deref for $name {
                type Target = crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl const Into<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type>> for $name {
                fn into(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<1, $type> {
                    self.0
                }
            }
        };

        ($matrix:ident, $type:ty, $type_r:ty, $name:ident, $rotation_type:ty, 2, $theta:expr, $doc: expr) => {
            #[doc = $doc]
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub(crate)struct $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>);

            impl $name {
                pub(crate) const fn new(wire_0: usize, wire_1: usize) -> $name {
                    $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Borrowed(&$matrix),
                        [wire_0, wire_1],
                        None,
                    ))
                }

                pub(crate) const fn new_classically_controlled(wire_0: usize, wire_1: usize, classical_control: usize) -> $name {
                    $name(crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator::new(
                        crate::util::s_cow::SCow::Borrowed(&$matrix),
                        [wire_0, wire_1],
                        Some(classical_control),
                    ))
                }

                pub(crate) const fn new_parameterized(theta: f64, wire_0: usize, wire_1: usize, classical_control: Option<usize>) -> $rotation_type {
                    if let Some(classical_control) = classical_control {
                        <$rotation_type>::new_classically_controlled(theta, wire_0, wire_1, classical_control)
                    } else {
                        <$rotation_type>::new(theta, wire_0, wire_1)
                    }
                }

                pub(crate)fn operator(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type> {
                    self.0
                }
            }

            impl const crate::toolbox::parameterized::Parameterized<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type_r>, $rotation_type> for $name {
                fn parameterized(&self, theta: f64) -> $rotation_type {
                    if let Some(classical_control) = self.classical_control() {
                        <$rotation_type>::new_classically_controlled(theta / $theta, self.wires()[0], self.wires()[1], classical_control)
                    } else {
                        <$rotation_type>::new(theta / $theta, self.wires()[0], self.wires()[1])
                    }
                }
            }

            impl const core::ops::Deref for $name {
                type Target = crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl const Into<crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type>> for $name {
                fn into(self) -> crate::runtime::const_sized::unitary_operator::ConstSizedUnitaryOperator<2, $type> {
                    self.0
                }
            }
        };
    }
}
