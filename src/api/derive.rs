use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::stringify;
use core::concat;
use core::f64::consts::PI;
use spin::Mutex;
use tinyvec::TinyVec;
use crate::api::{ClassicalRegister, QuantumRegister};
use crate::runtime::ket::Ket;
use crate::runtime::non_unitary::measurement::{Measurement, MeasurementBasis};
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
use crate::toolbox::parameterized::Parameterized;
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
pub struct Algorithm {
    pub(crate) ket: Ket,
    pub(crate) reg: Register,
    pub(crate) tools: Vec<TinyVec<[Tool; 1]>>,
    pub(crate) pos: usize,
    pub(crate) sub_pos: usize,
    pub(crate) steps: usize,
}

impl Algorithm {
    pub fn new<F: Fn(GateBuilder) -> GateBuilder>(f: F) -> Algorithm {
        let builder = GateBuilder(Mutex::new(0), Mutex::new(0), Mutex::new(Vec::new()));
        let mut builder = f(builder);
        Algorithm {
            ket: Ket::new(*builder.0.get_mut()).unwrap(),
            reg: Register::new(*builder.1.get_mut()),
            tools: builder.2.get_mut().clone(),
            pos: 0,
            sub_pos: 0,
            steps: 1,
        }
    }

    pub fn into_stepper(self, steps: usize) -> Algorithm {
        assert!(steps > 0);
        Algorithm {
            ket: self.ket,
            reg: self.reg,
            tools: self.tools,
            pos: self.pos,
            sub_pos: self.sub_pos,
            steps,
        }
    }

    pub fn step(&mut self) -> Option<(QuantumRegister, ClassicalRegister)> {
        let mut ket = self.ket.clone();
        let mut reg = self.reg.clone();
        self.sub_pos += 1;
        if let Some(col) = self.tools.get(self.pos) {
            for tool in col {
                let tool = if self.sub_pos == self.steps {
                    tool.clone()
                } else {
                    let part = self.sub_pos as f64 / self.steps as f64;
                    let theta = PI * part;
                    match tool {
                        Tool::Operator(o) => match o {
                            Operator::Hadamard(o) => o.parameterized(theta).into(),
                            Operator::PauliX(o) => o.parameterized(theta).into(),
                            Operator::PauliXRoot(o) => o.parameterized(theta).into(),
                            Operator::PauliY(o) => o.parameterized(theta).into(),
                            Operator::PauliZ(o) => o.parameterized(theta).into(),
                            Operator::Phase(o) => o.parameterized(theta).into(),
                            Operator::PhaseDagger(o) => o.parameterized(theta).into(),
                            Operator::PhaseRoot(o) => o.parameterized(theta).into(),
                            Operator::PhaseRootDagger(o) => o.parameterized(theta).into(),
                            Operator::Swap(o) => o.parameterized(theta).into(),
                            Operator::SwapRoot(o) => o.parameterized(theta).into(),
                        },
                        Tool::Rotation(o) => match o {
                            Rotation::Hadamard(o) => o.parameterized(theta).into(),
                            Rotation::X(o) => o.parameterized(theta).into(),
                            Rotation::Y(o) => o.parameterized(theta).into(),
                            Rotation::Z(o) => o.parameterized(theta).into(),
                            Rotation::PauliX(o) => o.parameterized(theta).into(),
                            Rotation::PauliY(o) => o.parameterized(theta).into(),
                            Rotation::PauliZ(o) => o.parameterized(theta).into(),
                            Rotation::Swap(o) => o.parameterized(theta).into(),
                        },
                        Tool::Controlled(o) => match o {
                            Controlled::ControlledHadamard(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPauliX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPauliXRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPauliY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPauliZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPhase(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPhaseDagger(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPhaseRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledPhaseRootDagger(o) => o.parameterized(theta).into(),
                            Controlled::ControlledSwap(o) => o.parameterized(theta).into(),
                            Controlled::ControlledSwapRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationHadamard(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationPauliX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationPauliY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationPauliZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledRotationSwap(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledHadamard(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPauliX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPauliXRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPauliY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPauliZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPhase(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPhaseDagger(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPhaseRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledPhaseRootDagger(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledSwap(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledSwapRoot(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationHadamard(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationPauliX(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationPauliY(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationPauliZ(o) => o.parameterized(theta).into(),
                            Controlled::ControlledControlledRotationSwap(o) => o.parameterized(theta).into(),
                        },
                        Tool::Measurement(m) => Tool::Measurement(*m),
                        Tool::Reset(r) => Tool::Reset(*r),
                        Tool::Custom(c) => Tool::Custom(c.clone()),
                        Tool::None => Tool::None,
                    }
                };

                ket = tool.apply(ket, &mut reg);
            }

            if self.sub_pos == self.steps {
                self.ket = ket.clone();
                self.reg = reg.clone();
                self.pos = self.pos + 1;
                self.sub_pos = 0;
            }

            Some((QuantumRegister(ket), ClassicalRegister(reg)))
        } else {
            None
        }
    }

    pub fn run(self) -> (QuantumRegister, ClassicalRegister) {
        let mut ket = self.ket;
        let mut reg = self.reg;
        for step in self.tools {
            for tool in step {
                ket = tool.apply(ket, &mut reg);
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

    fn push_col(&self, tool: Tool) {
        let mut tools = self.1.2.lock();
        tools.push(TinyVec::from([tool; 1]));
    }

    fn push(&self, tool: Tool) {
        let mut tools = self.1.2.lock();
        if let Some(col) = tools.last_mut() {
            col.push(tool);
        } else {
            tools.push(TinyVec::from([tool; 1]));
        }
    }
}

#[derive(Copy, Clone)]
pub struct Bit<'a>(usize, &'a GateBuilder);

impl<'a> Bit<'a> {
    fn idx(&self) -> usize {
        self.0
    }

    fn push_col(&self, tool: Tool) {
        let mut tools = self.1.2.lock();
        tools.push(TinyVec::from([tool; 1]));
    }

    fn push(&self, tool: Tool) {
        let mut tools = self.1.2.lock();
        if let Some(col) = tools.last_mut() {
            col.push(tool);
        } else {
            tools.push(TinyVec::from([tool; 1]));
        }
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
            qbit.push_col(<$type>::new(qbit.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: QBit, t_qbit: QBit) {
            c_qbit.push_col(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: QBit, c_qbit_1: QBit, t_qbit: QBit) {
            c_qbit_0.push_col(
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
            qbit_0.push_col(<$type>::new(qbit_0.idx(), qbit_1.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit.push_col(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: QBit, c_qbit_1: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit_0.push_col(
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
            qbit.push_col(<$type>::new(theta, qbit.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: QBit, t_qbit: QBit) {
            c_qbit.push_col(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: QBit, c_qbit_1: QBit, t_qbit: QBit) {
            c_qbit_0.push_col(
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
            qbit_0.push_col(<$type>::new(theta, qbit_0.idx(), qbit_1.idx()).into())
        }

        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit.push_col(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: QBit, c_qbit_1: QBit, t_qbit_0: QBit, t_qbit_1: QBit) {
            c_qbit_0.push_col(
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

pub fn measurement_x(qbit: QBit, bit: Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::X), Some(bit.idx())).into())
}

pub fn measurement_y(qbit: QBit, bit: Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::Y), Some(bit.idx())).into())
}

pub fn measurement_z(qbit: QBit, bit: Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::Z), Some(bit.idx())).into())
}

pub fn reset(qbit: QBit) {
    qbit.push_col(Reset::new(qbit.idx(), false).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_bell() {
        let mut algorithm = Algorithm::new(|gate_builder| {
            let a = gate_builder.qbit();
            let b = gate_builder.qbit();
            let c_a = gate_builder.bit();
            let c_b = gate_builder.bit();

            hadamard(a);
            controlled_pauli_x(a, b);

            measurement_z(a, c_a);
            measurement_z(b, c_b);

            gate_builder
        });

        let reg = algorithm.run().1;
        assert_eq!(reg.state().get(0).unwrap(), reg.state().get(1).unwrap())
    }

    fn bv_algorithm(hidden: Vec<bool>) {
        let mut algorithm = Algorithm::new(|gate_builder| {
            let qbits = (0..hidden.len()).map(|_| gate_builder.qbit()).collect::<Vec<_>>();
            let bits = (0..hidden.len()).map(|_| gate_builder.bit()).collect::<Vec<_>>();
            let target = gate_builder.qbit();

            hadamard(target);
            pauli_z(target);

            for qbit in qbits.iter() {
                hadamard(*qbit);
            }

            for idx in 0..qbits.len() {
                if *hidden.get(idx).unwrap() {
                    controlled_pauli_x(*qbits.get(idx).unwrap(), target);
                }
            }

            for qbit in qbits.iter() {
                hadamard(*qbit);
            }
            hadamard(target);

            for idx in 0..qbits.len() {
                measurement_z(*qbits.get(idx).unwrap(), *bits.get(idx).unwrap());
            }

            gate_builder
        });

        let (quantum_register, classical_register) = algorithm.run();

        for (hidden, measured) in hidden.iter().zip(classical_register.state().iter()) {
            assert_eq!(*hidden, *measured);
        }
    }

    #[test]
    fn test_bv_extreme() {
        bv_algorithm(Vec::from([
            true, true, false, true, false,
            true, true, false,  true, true,
            false, true, false, true, true,
            false, true, false, true, true,
            false, true, false, false, true,
        ]));
    }

    #[test]
    fn test_bv_large() {
        bv_algorithm(Vec::from([
            true, true, false, true, false,
            true, true, false, false, true,
            false, true,
        ]));
    }

    #[test]
    fn test_bv_medium() {
        bv_algorithm(Vec::from([
            true, true, false, true, false,
            true, true, false,
        ]));
    }

    #[test]
    fn test_bv_small() {
        bv_algorithm(Vec::from([
            true, true, false, true, false,
        ]));
    }
}
