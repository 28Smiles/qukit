use alloc::format;
use js_sys::Float64Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Complex[]")]
    pub type ComplexArray;

    #[wasm_bindgen(typescript_type = "AlgorithmGate[]")]
    pub type AlgorithmGateArray;
}

#[wasm_bindgen]
pub struct QuantumComputer(crate::quantum::computer::QuantumComputer);

#[wasm_bindgen]
impl QuantumComputer {
    /// Creates a new quantum computer instance
    #[wasm_bindgen(constructor)]
    pub fn new(q_bits: u32, seed: Option<u64>) -> QuantumComputer {
        QuantumComputer(crate::quantum::computer::QuantumComputer::new(q_bits, seed))
    }

    /// Returns the state of the quantum computer
    pub fn state(&self) -> ComplexArray {
        serde_wasm_bindgen::to_value(self.0.state()).unwrap().unchecked_into()
    }

    /// Returns the probabillity of all states
    pub fn amplitudes(&self) -> Float64Array {
        Float64Array::from(self.0.amplitudes().as_slice())
    }

    /// Returns the probalillites of each qbit
    pub fn probalillites(&self) -> Float64Array {
        Float64Array::from(self.0.probalillites().as_slice())
    }

    /// Returns the proballily of a qbit
    pub fn probability(&self, bit: u32) -> f64 {
        self.0.probability(bit)
    }
}

#[wasm_bindgen]
pub struct QuantumAlgorithm(crate::quantum::algorithm::QuantumAlgorithm);

#[wasm_bindgen]
impl QuantumAlgorithm {
    /// Creates a new quantum algorithm instance
    #[wasm_bindgen(constructor)]
    pub fn new(gates: AlgorithmGateArray, step_size: Option<u32>) -> QuantumAlgorithm {
        QuantumAlgorithm(crate::quantum::algorithm::QuantumAlgorithm::new(
            match serde_wasm_bindgen::from_value(gates.unchecked_into()) {
                Ok(value) => value,
                Err(error) => wasm_bindgen::throw_str(
                    &*format!("Error during deserialisation: {}", error)
                ),
            },
            step_size
        ))
    }

    /// Apply the current transformation to the quantum computer
    #[wasm_bindgen]
    pub fn apply(&mut self, qantum_computer: &mut QuantumComputer) -> bool {
        self.0.apply_next_gate(&mut qantum_computer.0)
    }

    /// Apply all transformations to the quantum computer
    #[wasm_bindgen]
    pub fn run(&mut self, qantum_computer: &mut QuantumComputer) {
        self.0.run(&mut qantum_computer.0)
    }

    /// The current position of the algorithm
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> u32 {
        self.0.position()
    }

    /// The current subposition between [0, step_size - 1]
    #[wasm_bindgen(getter)]
    pub fn subposition(&self) -> u32 {
        self.0.subposition()
    }
}
