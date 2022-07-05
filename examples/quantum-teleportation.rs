#![feature(generic_const_exprs)]

use qukit::quantum::algorithm::QuantumAlgorithm;
use qukit::quantum::computer::QuantumComputer;
use qukit::quantum::operator::OperatorType;
use qukit::quantum::operator::simple::controlled::Controlled;
use qukit::quantum::operator::simple::hadamard::Hadamard;
use qukit::quantum::operator::simple::pauli_x::PauliX;
use qukit::quantum::operator::simple::pauli_z::PauliZ;
use qukit::quantum::operator::simple::Simple;
use qukit::quantum::operator::special::measurement::{Measurement, MeasurementBasis};
use qukit::quantum::operator::special::Special;

fn main() {
    let mut quantum_computer = QuantumComputer::new(3, None);
    for _ in 0..12 {
        let mut algorithm = QuantumAlgorithm::new(Vec::new(), None);

        algorithm.add_gate(0, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0))));

        algorithm.add_gate(1, OperatorType::Simple(Simple::Hadamard(Hadamard::new(2))));
        algorithm.add_gate(2, OperatorType::Simple(Simple::ControlledPauliX(Controlled::new(2, PauliX::new(1)))));

        algorithm.add_gate(3, OperatorType::Simple(Simple::ControlledPauliX(Controlled::new(0, PauliX::new(1)))));
        algorithm.add_gate(4, OperatorType::Simple(Simple::Hadamard(Hadamard::new(0))));
        algorithm.add_gate(5, OperatorType::Special(Special::Measurement(Measurement::new(0, Some(MeasurementBasis::Z), None, None))));
        algorithm.add_gate(5, OperatorType::Special(Special::Measurement(Measurement::new(1, Some(MeasurementBasis::Z), None, None))));

        algorithm.add_gate(6, OperatorType::Simple(Simple::ControlledPauliX(Controlled::new(1, PauliX::new(2)))));
        algorithm.add_gate(7, OperatorType::Simple(Simple::ControlledPauliZ(Controlled::new(0, PauliZ::new(2)))));
        algorithm.run(&mut quantum_computer);
        println!("{}", quantum_computer.state());
        quantum_computer.reset();
    }
}
