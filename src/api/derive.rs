use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::stringify;
use core::concat;
use spin::Mutex;
use tinyvec::TinyVec;
use crate::api::{ClassicalRegister, QuantumRegister};
use crate::runtime::ket::Ket;
use crate::runtime::non_unitary::measurement::Measurement;
use crate::runtime::non_unitary::reset::Reset;
use crate::runtime::register::Register;
use crate::runtime::unitary::UnitaryOperator;
use crate::toolbox::controlled::Controlled;
use crate::toolbox::operator::c::C;
use crate::toolbox::operator::hadamard::Hadamard;
use crate::toolbox::operator::Operator;
use crate::toolbox::operator::pauli_x::PauliX;
use crate::toolbox::operator::pauli_y::PauliY;
use crate::toolbox::operator::pauli_z::PauliZ;
use crate::toolbox::operator::phase::{Phase, PhaseDagger};
use crate::toolbox::operator::phase_root::{PhaseRoot, PhaseRootDagger};
use crate::toolbox::operator::pauli_x_root::PauliXRoot;
use crate::toolbox::operator::swap::Swap;
use crate::toolbox::operator::swap_root::SwapRoot;
use crate::toolbox::rotation::pauli_x::RotationPauliX;
use crate::toolbox::rotation::pauli_y::RotationPauliY;
use crate::toolbox::rotation::pauli_z::RotationPauliZ;
use crate::toolbox::rotation::x::RotationX;
use crate::toolbox::rotation::y::RotationY;
use crate::toolbox::rotation::z::RotationZ;
use crate::toolbox::rotation::hadamard::RotationHadamard;
use crate::toolbox::rotation::swap::RotationSwap;
use crate::toolbox::rotation::Rotation;
use crate::toolbox::Tool;

#[derive(Clone)]
pub struct Algorithm(pub(crate) Ket, pub(crate) Register, pub(crate) Vec<TinyVec<[Tool; 1]>>);

impl Algorithm {
    pub fn new<F: Fn(GateBuilder) -> GateBuilder>(f: F) -> Algorithm {
        let builder = GateBuilder(Mutex::new(0), Mutex::new(0), Mutex::new(Vec::new()));
        let mut builder = f(builder);
        Algorithm(
            Ket::new(*builder.0.get_mut()).unwrap(),
            Register::new(*builder.1.get_mut()),
            builder.2.get_mut().clone()
        )
    }

    pub fn run(self) -> (QuantumRegister, ClassicalRegister) {
        let mut ket = self.0;
        let mut reg = self.1;
        for col in self.2 {
            for tool in col {
                ket = match tool {
                    Tool::Operator(o) => {
                        match o {
                            Operator::Hadamard(o) => o.operator().apply(ket, &mut reg),
                            Operator::PauliX(o) => o.operator().apply(ket, &mut reg),
                            Operator::PauliXRoot(o) => o.operator().apply(ket, &mut reg),
                            Operator::PauliY(o) => o.operator().apply(ket, &mut reg),
                            Operator::PauliZ(o) => o.operator().apply(ket, &mut reg),
                            Operator::Phase(o) => o.operator().apply(ket, &mut reg),
                            Operator::PhaseDagger(o) => o.operator().apply(ket, &mut reg),
                            Operator::PhaseRoot(o) => o.operator().apply(ket, &mut reg),
                            Operator::PhaseRootDagger(o) => o.operator().apply(ket, &mut reg),
                            Operator::Swap(o) => o.operator().apply(ket, &mut reg),
                            Operator::SwapRoot(o) => o.operator().apply(ket, &mut reg),
                        }
                    }
                    Tool::Rotation(o) => {
                        match o {
                            Rotation::Hadamard(o) => o.operator().apply(ket, &mut reg),
                            Rotation::X(o) => o.operator().apply(ket, &mut reg),
                            Rotation::Y(o) => o.operator().apply(ket, &mut reg),
                            Rotation::Z(o) => o.operator().apply(ket, &mut reg),
                            Rotation::PauliX(o) => o.operator().apply(ket, &mut reg),
                            Rotation::PauliY(o) => o.operator().apply(ket, &mut reg),
                            Rotation::PauliZ(o) => o.operator().apply(ket, &mut reg),
                            Rotation::Swap(o) => o.operator().apply(ket, &mut reg),
                        }
                    }
                    Tool::Controlled(o) => {
                        match o {
                            Controlled::ControlledHadamard(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPauliX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPauliXRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPauliY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPauliZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPhase(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPhaseDagger(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPhaseRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledPhaseRootDagger(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledSwap(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledSwapRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationHadamard(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationPauliX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationPauliY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationPauliZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledRotationSwap(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledHadamard(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPauliX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPauliXRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPauliY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPauliZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPhase(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPhaseDagger(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPhaseRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledPhaseRootDagger(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledSwap(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledSwapRoot(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationHadamard(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationPauliX(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationPauliY(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationPauliZ(o) => o.operator().apply(ket, &mut reg),
                            Controlled::ControlledControlledRotationSwap(o) => o.operator().apply(ket, &mut reg),
                        }
                    }
                    Tool::Measurement(o) => o.apply(ket, &mut reg),
                    Tool::Reset(o) => o.apply(ket, &mut reg),
                    Tool::Custom(o) => o.apply(ket, &mut reg),
                    Tool::None => ket,
                };
            }
        }

        (QuantumRegister(ket), ClassicalRegister(reg))
    }
}

#[derive(Copy, Clone)]
pub struct QBit<'a>(usize, &'a GateBuilder);

impl<'a> QBit<'a> {
    fn idx(&self) -> usize {
        self.0
    }

    fn add(&self, tool: Tool) {
        self.1.2.lock().push(TinyVec::from([tool; 1]));
    }
}

#[derive(Copy, Clone)]
pub struct Bit<'a>(usize, &'a GateBuilder);

impl<'a> Bit<'a> {
    fn idx(&self) -> usize {
        self.0
    }

    fn add(&self, tool: Tool) {
        self.1.2.lock().push(TinyVec::from([tool; 1]));
    }
}

pub struct GateBuilder(pub(crate) Mutex<usize>, pub(crate) Mutex<usize>, pub(crate) Mutex<Vec<TinyVec<[Tool; 1]>>>);
impl GateBuilder {
    pub fn qbit(&self) -> QBit {
        let mut bit_id = self.0.lock();
        let qbit = QBit(*bit_id.deref(), self);
        *bit_id.deref_mut() += 1;

        qbit
    }

    pub fn bit(&self) -> Bit {
        let mut bit_id = self.1.lock();
        let bit = Bit(*bit_id.deref(), self);
        *bit_id.deref_mut() += 1;

        bit
    }
}

macro_rules! impl_operator {
    ($name:ident, $cname:ident, $ccname:ident, $type:ty) => {
        impl_operator!(@operator, $name, $cname, $ccname, $type, 1);
    };

    ($name:ident, $cname:ident, $ccname:ident, $type:ty, 2) => {
        impl_operator!(@operator, $name, $cname, $ccname, $type, 2);
    };

    (@rotation, $name:ident, $cname:ident, $ccname:ident, $type:ty) => {
        impl_operator!(@rotation, $name, $cname, $ccname, $type, 1);
    };

    (@operator, $name:ident, $cname:ident, $ccname:ident, $type:ty, 1) => {
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(qbit: QBit) {
            qbit.add(<$type>::new(qbit.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: QBit, t_qbit: QBit) {
            c_qbit.add(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: QBit, c_qbit_1: QBit, t_qbit: QBit) {
            c_qbit_0.add(
                C::<3, C<2, $type, _>, _>::new(
                    c_qbit_0.idx(),
                    C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit.idx()))
                ).into()
            )
        }
    };

    (@operator, $name:ident, $cname:ident, $ccname:ident, $type:ty, 2) => {
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(qbit_0: QBit, qbit_1: QBit) {
            qbit_0.add(<$type>::new(qbit_0.idx(), qbit_1.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit.add(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: QBit, c_qbit_1: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit_0.add(
                C::<4, C<3, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx()))
                ).into()
            )
        }
    };

    (@rotation, $name:ident, $cname:ident, $ccname:ident, $type:ty, 1) => {
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(theta: f64, qbit: QBit) {
            qbit.add(<$type>::new(theta, qbit.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: QBit, t_qbit: QBit) {
            c_qbit.add(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: QBit, c_qbit_1: QBit, t_qbit: QBit) {
            c_qbit_0.add(
                C::<3, C<2, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit.idx()))
                ).into()
            )
        }
    };

    (@rotation, $name:ident, $cname:ident, $ccname:ident, $type:ty, 2) => {
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(theta: f64, qbit_0: QBit, qbit_1: QBit) {
            qbit_0.add(<$type>::new(theta, qbit_0.idx(), qbit_1.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit.add(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: QBit, c_qbit_1: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit_0.add(
                C::<4, C<3, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx()))
                ).into()
            )
        }
    };

    (@doc, @operator, $type:ty) => {
        concat!(
            "Applies the [",
            stringify!($type),
            "](",
            stringify!($type),
            ")-Gate to the qbit"
        )
    };

    (@doc, @controlled, $type:ty) => {
        concat!(
            "Applies the controlled version of [",
            stringify!($type),
            "](",
            stringify!($type),
            ")-Gate to the qbit"
        )
    };

    (@doc, @controlledcontrolled, $type:ty) => {
        concat!(
            "Applies the two qbit controlled version of [",
            stringify!($type),
            "](",
            stringify!($type),
            ")-Gate to the qbit"
        )
    };
}

impl_operator!(hadamard, controlled_hadamard, controlled_controlled_hadamard, Hadamard);
impl_operator!(pauli_x, controlled_pauli_x, controlled_controlled_pauli_x, PauliX);
impl_operator!(pauli_y, controlled_pauli_y, controlled_controlled_pauli_y, PauliY);
impl_operator!(pauli_z, controlled_pauli_z, controlled_controlled_pauli_z, PauliZ);
impl_operator!(phase, controlled_phase, controlled_controlled_phase, Phase);
impl_operator!(phase_dagger, controlled_phase_dagger, controlled_controlled_phase_dagger, PhaseDagger);
impl_operator!(phase_root, controlled_phase_root, controlled_controlled_phase_root, PhaseRoot);
impl_operator!(phase_root_dagger, controlled_phase_root_dagger, controlled_controlled_phase_root_dagger, PhaseRootDagger);
impl_operator!(pauli_x_root, controlled_pauli_x_root, controlled_controlled_pauli_x_root, PauliXRoot);
impl_operator!(swap, controlled_swap, controlled_controlled_swap, Swap, 2);
impl_operator!(swap_root, controlled_swap_root, controlled_controlled_swap_root, SwapRoot, 2);

impl_operator!(@rotation, rotation_hadamard, controlled_rotation_hadamard, controlled_controlled_rotation_hadamard, RotationHadamard);
impl_operator!(@rotation, rotation_pauli_x, controlled_rotation_pauli_x, controlled_controlled_rotation_pauli_x, RotationPauliX);
impl_operator!(@rotation, rotation_pauli_y, controlled_rotation_pauli_y, controlled_controlled_rotation_pauli_y, RotationPauliY);
impl_operator!(@rotation, rotation_pauli_z, controlled_rotation_pauli_z, controlled_controlled_rotation_pauli_z, RotationPauliZ);
impl_operator!(@rotation, rotation_x, controlled_rotation_x, controlled_controlled_rotation_x, RotationX);
impl_operator!(@rotation, rotation_y, controlled_rotation_y, controlled_controlled_rotation_y, RotationY);
impl_operator!(@rotation, rotation_z, controlled_rotation_z, controlled_controlled_rotation_z, RotationZ);
impl_operator!(@rotation, rotation_swap, controlled_rotation_swap, controlled_controlled_rotation_swap, RotationSwap, 2);

pub fn measurement(qbit: QBit, bit: Bit) {
    qbit.add(Measurement::new(qbit.idx(), None, Some(bit.idx())).into())
}

pub fn reset(qbit: QBit) {
    qbit.add(Reset::new(qbit.idx(), false).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test() {
        let algorithm = Algorithm::new(|gate_builder| {
            let a = gate_builder.qbit();
            let b = gate_builder.qbit();
            let c_a = gate_builder.bit();
            let c_b = gate_builder.bit();

            hadamard(a);
            controlled_pauli_x(a, b);

            measurement(a, c_a);
            measurement(b, c_b);

            gate_builder
        });

        println!("{}", algorithm.run().0);
    }
}
