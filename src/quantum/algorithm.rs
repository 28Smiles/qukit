use alloc::vec::Vec;
use core::f64::consts::PI;
use crate::quantum::computer::QuantumComputer;
use crate::quantum::operator::OperatorType;
use crate::quantum::gate::sized::SizedGate;

#[cfg(feature = "wasm-pack")]
use tsify::Tsify;

#[cfg_attr(feature = "wasm-pack", derive(Tsify))]
#[cfg_attr(feature = "wasm-pack", tsify(from_wasm_abi))]
#[cfg_attr(feature = "wasm-pack", derive(serde::Deserialize))]
#[cfg_attr(feature = "wasm-pack", serde(rename_all = "camelCase"))]
pub struct AlgorithmGate {
    position: u32,
    gate: OperatorType,
}

impl AlgorithmGate {
    fn new(position: u32, gate: OperatorType) -> AlgorithmGate {
        AlgorithmGate {
            position,
            gate,
        }
    }
}

pub struct QuantumAlgorithm {
    gates: Vec<AlgorithmGate>,
    position: u32,
    subposition: u32,
    step_size: u32,
    last_gate: Vec<SizedGate>,
    last_idx: usize,
}

impl QuantumAlgorithm {
    pub fn new(mut gates: Vec<AlgorithmGate>, step_size: Option<u32>) -> QuantumAlgorithm {
        gates.sort_by_cached_key(|gate| gate.position);
        QuantumAlgorithm {
            gates,
            position: 0,
            subposition: 0,
            step_size: step_size.unwrap_or(1),
            last_gate: Vec::new(),
            last_idx: 0,
        }
    }

    pub fn apply_next_gate(&mut self, computer: &mut QuantumComputer) -> bool {
        if self.last_idx == self.gates.len() {
            return false
        }

        if self.step_size <= 1 {
            for i in self.last_idx..self.gates.len() {
                if let Some(gate) = self.gates.get(i) {
                    if gate.position > self.position {
                        self.last_idx = i;
                        break
                    }
                    if gate.position == self.position {
                        // Add gate to last_gates
                        gate.gate.apply(computer);
                    } else {
                        self.last_idx = self.gates.len();
                        return false
                    }
                } else {
                    self.last_idx = i;
                    break
                }
            }
        } else {
            if self.last_gate.is_empty() {
                // Prepare Gates
                let theta = PI / (self.step_size as f64);
                for i in self.last_idx..self.gates.len() {
                    if let Some(gate) = self.gates.get(i) {
                        if gate.position > self.position {
                            self.last_idx = i;
                            break;
                        }
                        if gate.position == self.position {
                            // Add gate to last_gates
                            match &gate.gate {
                                OperatorType::Special(gate) => gate.apply(computer),
                                OperatorType::Rotation(gate) => self.last_gate.push((gate).get_parameterized(theta)),
                                OperatorType::Simple(gate) => self.last_gate.push((gate).get_parameterized(theta)),
                            }
                        }
                    } else {
                        self.last_idx = i;
                        break;
                    }
                }
            }
            if self.last_gate.is_empty() {
                return false
            }

            for gate in &self.last_gate {
                gate.apply(computer);
            }
        }

        self.subposition += 1;
        if self.next_step_is_full() {
            self.subposition = 0;
            self.position += 1;
            self.last_gate.clear();
        }

        true
    }

    pub fn run(&mut self, computer: &mut QuantumComputer) {
        while self.apply_next_gate(computer) {

        }
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn subposition(&self) -> u32 {
        self.subposition
    }

    pub(crate) fn next_step_is_full(&self) -> bool {
        self.subposition >= self.step_size
    }

    pub fn add_gate(&mut self, pos: u32, gate: OperatorType) {
        self.gates.push(AlgorithmGate::new(pos, gate));
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use float_cmp::assert_approx_eq;
    use crate::quantum::algorithm::{AlgorithmGate, QuantumAlgorithm};
    use crate::quantum::computer::QuantumComputer;
    use crate::quantum::operator::OperatorType;
    use crate::quantum::operator::simple::hadamard::Hadamard;
    use crate::quantum::operator::simple::Simple;

    #[test]
    fn test_algorithm() {
        let mut algorithm = QuantumAlgorithm::new(
            Vec::from([
                AlgorithmGate::new(0, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0)))),
                AlgorithmGate::new(1, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0)))),
            ]),
            Some(256),
        );
        let mut quantum_computer = QuantumComputer::new(1, None);
        for _ in 0..256 {
            assert!(algorithm.apply_next_gate(&mut quantum_computer));
        }
        for amplitude in quantum_computer.get_state().amplitudes() {
            assert_approx_eq!(f64, amplitude, 0.5, epsilon = 0.00001);
        }
        for _ in 0..256 {
            assert!(algorithm.apply_next_gate(&mut quantum_computer));
        }
        assert_approx_eq!(f64, *quantum_computer.get_state().amplitudes().get(0).unwrap(), 1.0, epsilon = 0.00001);
        assert_approx_eq!(f64, *quantum_computer.get_state().amplitudes().get(1).unwrap(), 0.0, epsilon = 0.00001);
        assert!(!algorithm.apply_next_gate(&mut quantum_computer));
    }

    #[test]
    fn test_algorithm_full() {
        let mut algorithm = QuantumAlgorithm::new(
            Vec::from([
                AlgorithmGate::new(0, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0)))),
                AlgorithmGate::new(1, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0)))),
            ]),
            None,
        );
        let mut quantum_computer = QuantumComputer::new(1, None);
        assert!(algorithm.apply_next_gate(&mut quantum_computer));
        for amplitude in quantum_computer.get_state().amplitudes() {
            assert_approx_eq!(f64, amplitude, 0.5, epsilon = 0.00001);
        }
        assert!(algorithm.apply_next_gate(&mut quantum_computer));
        assert_approx_eq!(f64, *quantum_computer.get_state().amplitudes().get(0).unwrap(), 1.0, epsilon = 0.00001);
        assert_approx_eq!(f64, *quantum_computer.get_state().amplitudes().get(1).unwrap(), 0.0, epsilon = 0.00001);
        assert!(!algorithm.apply_next_gate(&mut quantum_computer));
    }
}
