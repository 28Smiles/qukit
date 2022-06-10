use alloc::vec::Vec;
use crate::quantum::computer::QuantumComputer;
use crate::quantum::gate::matrix::const_sized::Gate;
use crate::quantum::gate::matrix::dynamic::DGate;

pub enum SizedGate {
    G1(Gate<1>, [u32; 1]),
    G2(Gate<2>, [u32; 2]),
    G3(Gate<3>, [u32; 3]),
    G4(Gate<4>, [u32; 4]),
    GD(DGate, Vec<u32>),
}

impl SizedGate {
    pub(crate) fn apply(&self, computer: &mut QuantumComputer) {
        match self {
            SizedGate::G1(gate, qbits) => gate.apply(computer, *qbits),
            SizedGate::G2(gate, qbits) => gate.apply(computer, *qbits),
            SizedGate::G3(gate, qbits) => gate.apply(computer, *qbits),
            SizedGate::G4(gate, qbits) => gate.apply(computer, *qbits),
            SizedGate::GD(gate, qbits) => gate.apply(computer, qbits.as_slice()),
        }
    }
}
