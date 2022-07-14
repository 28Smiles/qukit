use alloc::vec::Vec;
use core::stringify;
use core::concat;
use alloc::format;
use tinyvec::TinyVec;
use paste::paste;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use crate::api::{ClassicalRegister, QuantumRegister};
use crate::runtime::ket::Ket;
use crate::runtime::non_unitary::measurement::Measurement;
use crate::runtime::non_unitary::measurement::MeasurementBasis;
use crate::runtime::non_unitary::reset::Reset;
use crate::runtime::register::Register;
use crate::toolbox::operator::c::C;
use crate::toolbox::operator::hadamard::Hadamard;
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
use crate::toolbox::Tool;

#[wasm_bindgen]
#[derive(Clone)]
pub struct AlgorithmResult(QuantumRegister, ClassicalRegister);

#[wasm_bindgen]
impl AlgorithmResult {
    #[wasm_bindgen(js_name = quantumRegister)]
    pub fn quantum_register(&self) -> QuantumRegister {
        self.0.clone()
    }
    #[wasm_bindgen(js_name = classicalRegister)]
    pub fn classical_register(&self) -> ClassicalRegister {
        self.1.clone()
    }
    pub fn log(&self) {
        log(&*format!("{}\n{}", &self.0, &self.1));
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Algorithm(super::derive::Algorithm);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(typescript_type = "QBit")]
    pub type QBitType;

    #[wasm_bindgen(typescript_type = "Bit")]
    pub type BitType;
}

#[wasm_bindgen]
impl Algorithm {
    pub fn run(self) -> AlgorithmResult {
        let (quantum_register, classical_register) = self.0.run();

        AlgorithmResult(quantum_register, classical_register)
    }

    pub fn step(&mut self) -> Option<AlgorithmResult> {
        if let Some((quantum_register, classical_register)) = self.0.step() {
            Some(AlgorithmResult(quantum_register, classical_register))
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = intoStepper)]
    pub fn into_stepper(self, steps: usize) -> Algorithm {
        Algorithm(self.0.into_stepper(steps))
    }
}

#[wasm_bindgen]
pub struct QBit(usize, *mut GateBuilder);

impl QBit {
    fn idx(&self) -> usize {
        self.0
    }

    fn push_col(&self, tool: Tool) {
        (unsafe { &mut *self.1 }).2.push(TinyVec::from([tool; 1]));
    }

    fn push(&self, tool: Tool) {
        if let Some(col) = (unsafe { &mut *self.1 }).2.last_mut() {
            col.push(tool);
        } else {
            self.push_col(tool);
        }
    }
}

#[wasm_bindgen]
pub struct Bit(usize, *mut GateBuilder);

impl Bit {
    fn idx(&self) -> usize {
        self.0
    }

    fn push_col(&self, tool: Tool) {
        (unsafe { &mut *self.1 }).2.push(TinyVec::from([tool; 1]));
    }

    fn push(&self, tool: Tool) {
        if let Some(col) = (unsafe { &mut *self.1 }).2.last_mut() {
            col.push(tool);
        } else {
            self.push_col(tool);
        }
    }
}

#[wasm_bindgen]
pub struct GateBuilder(usize, usize, Vec<TinyVec<[Tool; 1]>>);

#[wasm_bindgen]
impl GateBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GateBuilder {
        #[cfg(feature="console_error_panic_hook")]
        console_error_panic_hook::set_once();

        GateBuilder(0, 0, Vec::new())
    }

    #[wasm_bindgen(js_name = intoAlgorithm)]
    pub fn into_algorithm(self) -> Algorithm {
        let ket = Ket::new(self.0).unwrap();
        let reg = Register::new(self.1);
        let tools = self.2;
        Algorithm(super::derive::Algorithm {
            ket,
            reg,
            tools,
            pos: 0,
            sub_pos: 0,
            steps: 1,
        })
    }

    pub fn qbit(&mut self) -> QBit {
        let qbit = QBit(self.0, self);
        self.0 += 1;

        qbit
    }

    pub fn qbits(&mut self, size: usize) -> Vec<QBitType> {
        (0..size).map(|_| {
            self.qbit()
        }).map(JsValue::from).map(JsValue::unchecked_into).collect()
    }

    pub fn bit(&mut self) -> Bit {
        let bit = Bit(self.1, self);
        self.1 += 1;

        bit
    }

    pub fn bits(&mut self, size: usize) -> Vec<BitType> {
        (0..size).map(|_| {
            self.bit()
        }).map(JsValue::from).map(JsValue::unchecked_into).collect()
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
        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(qbit: &QBit) {
            qbit.push_col(<$type>::new(qbit.idx()).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: &QBit, t_qbit: &QBit) {
            c_qbit.push_col(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit.idx())).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit: &QBit) {
            c_qbit_0.push_col(
                C::<3, C<2, $type, _>, _>::new(
                    c_qbit_0.idx(),
                    C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit.idx()))
                ).into()
            )
        }

        paste! {
            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step>](qbit: &QBit) {
                qbit.push(<$type>::new(qbit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlled, $type)]
            pub fn [<$cname _same_step>](c_qbit: &QBit, t_qbit: &QBit) {
                c_qbit.push(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit.idx())).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
            pub fn [<$ccname _same_step>](c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit: &QBit) {
                c_qbit_0.push(
                    C::<3, C<2, $type, _>, _>::new(
                        c_qbit_0.idx(),
                        C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit.idx()))
                    ).into()
                )
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step_classically_controlled>](qbit: &QBit, bit: &Bit) {
                qbit.push(<$type>::new_classically_controlled(qbit.idx(), bit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _classically_controlled>](qbit: &QBit, bit: &Bit) {
                qbit.push(<$type>::new_classically_controlled(qbit.idx(), bit.idx()).into())
            }
        }
    };

    (@operator, $name:ident, $cname:ident, $ccname:ident, $type:ty, 2) => {
        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(qbit_0: &QBit, qbit_1: &QBit) {
            qbit_0.push_col(<$type>::new(qbit_0.idx(), qbit_1.idx()).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(c_qbit: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
            c_qbit.push_col(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
            c_qbit_0.push_col(
                C::<4, C<3, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx()))
                ).into()
            )
        }

        paste! {
            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step>](qbit_0: &QBit, qbit_1: &QBit) {
                qbit_0.push(<$type>::new(qbit_0.idx(), qbit_1.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlled, $type)]
            pub fn [<$cname _same_step>](c_qbit: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
                c_qbit.push(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx())).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
            pub fn [<$ccname _same_step>](c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
                c_qbit_0.push(
                    C::<4, C<3, $type, _>, _>::new(
                        c_qbit_0.0,
                        C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(t_qbit_0.idx(), t_qbit_1.idx()))
                    ).into()
                )
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step_classically_controlled>](qbit_0: &QBit, qbit_1: &QBit, bit: &Bit) {
                qbit_0.push(<$type>::new_classically_controlled(qbit_0.idx(), qbit_1.idx(), bit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _classically_controlled>](qbit_0: &QBit, qbit_1: &QBit, bit: &Bit) {
                qbit_0.push_col(<$type>::new_classically_controlled(qbit_0.idx(), qbit_1.idx(), bit.idx()).into())
            }
        }
    };

    (@rotation, $name:ident, $cname:ident, $ccname:ident, $type:ty, 1) => {
        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(theta: f64, qbit: &QBit) {
            qbit.push_col(<$type>::new(theta, qbit.idx()).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: &QBit, t_qbit: &QBit) {
            c_qbit.push_col(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit.idx())).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit: &QBit) {
            c_qbit_0.push_col(
                C::<3, C<2, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit.idx()))
                ).into()
            )
        }

        paste! {
            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step>](theta: f64, qbit: &QBit) {
                qbit.push(<$type>::new(theta, qbit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlled, $type)]
            pub fn [<$cname _same_step>](theta: f64, c_qbit: &QBit, t_qbit: &QBit) {
                c_qbit.push(C::<2, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit.idx())).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
            pub fn [<$ccname _same_step>](theta: f64, c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit: &QBit) {
                c_qbit_0.push(
                    C::<3, C<2, $type, _>, _>::new(
                        c_qbit_0.0,
                        C::<2, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit.idx()))
                    ).into()
                )
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step_classically_controlled>](theta: f64, qbit: &QBit, bit: &Bit) {
                qbit.push(<$type>::new_classically_controlled(theta, qbit.idx(), bit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _classically_controlled>](theta: f64, qbit: &QBit, bit: &Bit) {
                qbit.push_col(<$type>::new_classically_controlled(theta, qbit.idx(), bit.idx()).into())
            }
        }
    };

    (@rotation, $name:ident, $cname:ident, $ccname:ident, $type:ty, 2) => {
        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @operator, $type)]
        pub fn $name(theta: f64, qbit_0: &QBit, qbit_1: &QBit) {
            qbit_0.push_col(<$type>::new(theta, qbit_0.idx(), qbit_1.idx()).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlled, $type)]
        pub fn $cname(theta: f64, c_qbit: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
            c_qbit.push_col(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx())).into())
        }

        #[wasm_bindgen]
        #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
        pub fn $ccname(theta: f64, c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
            c_qbit_0.push_col(
                C::<4, C<3, $type, _>, _>::new(
                    c_qbit_0.0,
                    C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx()))
                ).into()
            )
        }

        paste! {
            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step>](theta: f64, qbit_0: &QBit, qbit_1: &QBit) {
                qbit_0.push(<$type>::new(theta, qbit_0.idx(), qbit_1.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlled, $type)]
            pub fn [<$cname _same_step>](theta: f64, c_qbit: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
                c_qbit.push(C::<3, $type, _>::new(c_qbit.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx())).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @controlledcontrolled, $type)]
            pub fn [<$ccname _same_step>](theta: f64, c_qbit_0: &QBit, c_qbit_1: &QBit, t_qbit_0: &QBit, t_qbit_1: &QBit) {
                c_qbit_0.push(
                    C::<4, C<3, $type, _>, _>::new(
                        c_qbit_0.0,
                        C::<3, $type, _>::new(c_qbit_1.idx(), <$type>::new(theta, t_qbit_0.idx(), t_qbit_1.idx()))
                    ).into()
                )
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _same_step_classically_controlled>](theta: f64, qbit_0: &QBit, qbit_1: &QBit, bit: &Bit) {
                qbit_0.push(<$type>::new_classically_controlled(theta, qbit_0.idx(), qbit_1.idx(), bit.idx()).into())
            }

            #[wasm_bindgen]
            #[doc = impl_operator!(@doc, @operator, $type)]
            pub fn [<$name _classically_controlled>](theta: f64, qbit_0: &QBit, qbit_1: &QBit, bit: &Bit) {
                qbit_0.push_col(<$type>::new_classically_controlled(theta, qbit_0.idx(), qbit_1.idx(), bit.idx()).into())
            }
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

#[wasm_bindgen]
pub fn measurement_x(qbit: &QBit, bit: &Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::X), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn measurement_y(qbit: &QBit, bit: &Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::Y), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn measurement_z(qbit: &QBit, bit: &Bit) {
    qbit.push_col(Measurement::new(qbit.idx(), Some(MeasurementBasis::Z), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn measurement_x_same_step(qbit: &QBit, bit: &Bit) {
    qbit.push(Measurement::new(qbit.idx(), Some(MeasurementBasis::X), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn measurement_y_same_step(qbit: &QBit, bit: &Bit) {
    qbit.push(Measurement::new(qbit.idx(), Some(MeasurementBasis::Y), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn measurement_z_same_step(qbit: &QBit, bit: &Bit) {
    qbit.push(Measurement::new(qbit.idx(), Some(MeasurementBasis::Z), Some(bit.idx())).into())
}

#[wasm_bindgen]
pub fn reset(qbit: &QBit) {
    qbit.push_col(Reset::new(qbit.idx(), false).into())
}

#[wasm_bindgen]
pub fn reset_same_step(qbit: &QBit) {
    qbit.push(Reset::new(qbit.idx(), false).into())
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

            measurement_z(a, c_a);
            measurement_z(b, c_b);
        });

        println!("{}", algorithm.run().0);
    }
}
