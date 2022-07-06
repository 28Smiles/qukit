use crate::complex::Complex;
use crate::toolbox::operator::c::C;
use crate::toolbox::operator::hadamard::Hadamard;
use crate::toolbox::operator::pauli_x::PauliX;
use crate::toolbox::operator::pauli_x_root::PauliXRoot;
use crate::toolbox::operator::pauli_y::PauliY;
use crate::toolbox::operator::pauli_z::PauliZ;
use crate::toolbox::operator::phase::{Phase, PhaseDagger};
use crate::toolbox::operator::phase_root::{PhaseRoot, PhaseRootDagger};
use crate::toolbox::operator::swap::Swap;
use crate::toolbox::operator::swap_root::SwapRoot;
use crate::toolbox::rotation::hadamard::RotationHadamard;
use crate::toolbox::rotation::pauli_x::RotationPauliX;
use crate::toolbox::rotation::pauli_y::RotationPauliY;
use crate::toolbox::rotation::pauli_z::RotationPauliZ;
use crate::toolbox::rotation::swap::RotationSwap;
use crate::toolbox::rotation::x::RotationX;
use crate::toolbox::rotation::y::RotationY;
use crate::toolbox::rotation::z::RotationZ;
use crate::toolbox::Tool;

#[derive(Copy, Clone)]
pub(crate)enum Controlled {
    ControlledHadamard(C<2, Hadamard, f64>),
    ControlledPauliX(C<2, PauliX, f64>),
    ControlledPauliXRoot(C<2, PauliXRoot, Complex>),
    ControlledPauliY(C<2, PauliY, Complex>),
    ControlledPauliZ(C<2, PauliZ, f64>),
    ControlledPhase(C<2, Phase, Complex>),
    ControlledPhaseDagger(C<2, PhaseDagger, Complex>),
    ControlledPhaseRoot(C<2, PhaseRoot, Complex>),
    ControlledPhaseRootDagger(C<2, PhaseRootDagger, Complex>),
    ControlledSwap(C<3, Swap, f64>),
    ControlledSwapRoot(C<3, SwapRoot, Complex>),

    ControlledRotationHadamard(C<2, RotationHadamard, Complex>),
    ControlledRotationX(C<2, RotationX, Complex>),
    ControlledRotationY(C<2, RotationY, f64>),
    ControlledRotationZ(C<2, RotationZ, Complex>),
    ControlledRotationPauliX(C<2, RotationPauliX, Complex>),
    ControlledRotationPauliY(C<2, RotationPauliY, Complex>),
    ControlledRotationPauliZ(C<2, RotationPauliZ, Complex>),
    ControlledRotationSwap(C<3, RotationSwap, Complex>),

    ControlledControlledHadamard(C<3, C<2, Hadamard, f64>, f64>),
    ControlledControlledPauliX(C<3, C<2, PauliX, f64>, f64>),
    ControlledControlledPauliXRoot(C<3, C<2, PauliXRoot, Complex>, Complex>),
    ControlledControlledPauliY(C<3, C<2, PauliY, Complex>, Complex>),
    ControlledControlledPauliZ(C<3, C<2, PauliZ, f64>, f64>),
    ControlledControlledPhase(C<3, C<2, Phase, Complex>, Complex>),
    ControlledControlledPhaseDagger(C<3, C<2, PhaseDagger, Complex>, Complex>),
    ControlledControlledPhaseRoot(C<3, C<2, PhaseRoot, Complex>, Complex>),
    ControlledControlledPhaseRootDagger(C<3, C<2, PhaseRootDagger, Complex>, Complex>),
    ControlledControlledSwap(C<4, C<3, Swap, f64>, f64>),
    ControlledControlledSwapRoot(C<4, C<3, SwapRoot, Complex>, Complex>),

    ControlledControlledRotationHadamard(C<3, C<2, RotationHadamard, Complex>, Complex>),
    ControlledControlledRotationX(C<3, C<2, RotationX, Complex>, Complex>),
    ControlledControlledRotationY(C<3, C<2, RotationY, f64>, f64>),
    ControlledControlledRotationZ(C<3, C<2, RotationZ, Complex>, Complex>),
    ControlledControlledRotationPauliX(C<3, C<2, RotationPauliX, Complex>, Complex>),
    ControlledControlledRotationPauliY(C<3, C<2, RotationPauliY, Complex>, Complex>),
    ControlledControlledRotationPauliZ(C<3, C<2, RotationPauliZ, Complex>, Complex>),
    ControlledControlledRotationSwap(C<4, C<3, RotationSwap, Complex>, Complex>),
}

macro_rules! impl_from_trait {
    ($name:ident, $type:ty) => {
        impl From<$type> for Controlled {
            fn from(value: $type) -> Self {
                Controlled::$name(value)
            }
        }
        impl From<$type> for Tool {
            fn from(value: $type) -> Self {
                Tool::Controlled(Controlled::$name(value))
            }
        }
    };
}

impl_from_trait!(ControlledHadamard, C<2, Hadamard, f64>);
impl_from_trait!(ControlledPauliX, C<2, PauliX, f64>);
impl_from_trait!(ControlledPauliXRoot, C<2, PauliXRoot, Complex>);
impl_from_trait!(ControlledPauliY, C<2, PauliY, Complex>);
impl_from_trait!(ControlledPauliZ, C<2, PauliZ, f64>);
impl_from_trait!(ControlledPhase, C<2, Phase, Complex>);
impl_from_trait!(ControlledPhaseDagger, C<2, PhaseDagger, Complex>);
impl_from_trait!(ControlledPhaseRoot, C<2, PhaseRoot, Complex>);
impl_from_trait!(ControlledPhaseRootDagger, C<2, PhaseRootDagger, Complex>);
impl_from_trait!(ControlledSwap, C<3, Swap, f64>);
impl_from_trait!(ControlledSwapRoot, C<3, SwapRoot, Complex>);

impl_from_trait!(ControlledRotationHadamard, C<2, RotationHadamard, Complex>);
impl_from_trait!(ControlledRotationX, C<2, RotationX, Complex>);
impl_from_trait!(ControlledRotationY, C<2, RotationY, f64>);
impl_from_trait!(ControlledRotationZ, C<2, RotationZ, Complex>);
impl_from_trait!(ControlledRotationPauliX, C<2, RotationPauliX, Complex>);
impl_from_trait!(ControlledRotationPauliY, C<2, RotationPauliY, Complex>);
impl_from_trait!(ControlledRotationPauliZ, C<2, RotationPauliZ, Complex>);
impl_from_trait!(ControlledRotationSwap, C<3, RotationSwap, Complex>);

impl_from_trait!(ControlledControlledHadamard, C<3, C<2, Hadamard, f64>, f64>);
impl_from_trait!(ControlledControlledPauliX, C<3, C<2, PauliX, f64>, f64>);
impl_from_trait!(ControlledControlledPauliXRoot, C<3, C<2, PauliXRoot, Complex>, Complex>);
impl_from_trait!(ControlledControlledPauliY, C<3, C<2, PauliY, Complex>, Complex>);
impl_from_trait!(ControlledControlledPauliZ, C<3, C<2, PauliZ, f64>, f64>);
impl_from_trait!(ControlledControlledPhase, C<3, C<2, Phase, Complex>, Complex>);
impl_from_trait!(ControlledControlledPhaseDagger, C<3, C<2, PhaseDagger, Complex>, Complex>);
impl_from_trait!(ControlledControlledPhaseRoot, C<3, C<2, PhaseRoot, Complex>, Complex>);
impl_from_trait!(ControlledControlledPhaseRootDagger, C<3, C<2, PhaseRootDagger, Complex>, Complex>);
impl_from_trait!(ControlledControlledSwap, C<4, C<3, Swap, f64>, f64>);
impl_from_trait!(ControlledControlledSwapRoot, C<4, C<3, SwapRoot, Complex>, Complex>);

impl_from_trait!(ControlledControlledRotationHadamard, C<3, C<2, RotationHadamard, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationX, C<3, C<2, RotationX, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationY, C<3, C<2, RotationY, f64>, f64>);
impl_from_trait!(ControlledControlledRotationZ, C<3, C<2, RotationZ, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationPauliX, C<3, C<2, RotationPauliX, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationPauliY, C<3, C<2, RotationPauliY, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationPauliZ, C<3, C<2, RotationPauliZ, Complex>, Complex>);
impl_from_trait!(ControlledControlledRotationSwap, C<4, C<3, RotationSwap, Complex>, Complex>);
