use core::fmt::{Display, Formatter};
use crate::runtime::ket::Ket;
use crate::runtime::register::Register;
use crate::complex::Complex;

pub mod derive;
#[cfg(feature = "wasm-bindgen")]
pub mod derive_js;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone)]
pub struct QuantumRegister(pub(crate) Ket);

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Complex[]")]
    pub type ComplexArray;
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
impl QuantumRegister {
    /// Returns the array of all states
    pub fn states(&self) -> ComplexArray {
        use wasm_bindgen::JsCast;

        serde_wasm_bindgen::to_value(self.0.state()).unwrap().unchecked_into()
    }

    /// Returns the states of this index
    pub fn state(&self, idx: usize) -> Option<Complex> {
        self.0.state().get(idx).map(|c| *c)
    }

    /// Returns the probability of all states
    pub fn amplitudes(&self) -> js_sys::Float64Array {
        js_sys::Float64Array::from(self.0.amplitudes().as_slice())
    }

    /// Returns the probabilities of each qbit
    pub fn probabilities(&self) -> js_sys::Float64Array {
        js_sys::Float64Array::from(self.0.probabilities().as_slice())
    }

    /// Returns the probability of a qbit
    pub fn probability(&self, bit: usize) -> f64 {
        self.0.probability(bit)
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone)]
pub struct ClassicalRegister(pub(crate) Register);

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
impl ClassicalRegister {
    pub fn state(&self) -> alloc::vec::Vec<js_sys::Boolean> {
        self.0.bits().iter().map(|v| js_sys::Boolean::from(*v)).collect()
    }
}

impl Display for QuantumRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for ClassicalRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
