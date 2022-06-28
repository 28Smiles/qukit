use core::f64::consts::PI;
use crate::quantum::operator::simple::controlled::Controlled;
use crate::quantum::operator::simple::hadamard::Hadamard;
use crate::quantum::operator::simple::pauli_x::PauliX;
use crate::quantum::operator::simple::pauli_y::PauliY;
use crate::quantum::operator::simple::pauli_z::PauliZ;
use crate::quantum::operator::traits::{ApplyGate, Parameterized, ToGate};
use crate::quantum::computer::QuantumComputer;
use float_cmp::{approx_eq, assert_approx_eq};
use crate::complex::Complex;
use crate::quantum::gate::matrix::const_sized::Gate;
use test::Bencher;

extern crate test;

#[test]
fn pauli_x_1_qbit() {
    let mut computer = QuantumComputer::new(1, None);
    PauliX::new(0).apply(&mut computer);
    let c0 = computer.get_state().vec.get(0).unwrap();
    assert_approx_eq!(f64, c0.re(), 0.0, epsilon = 0.00001);
    assert_approx_eq!(f64, c0.im(), 0.0, epsilon = 0.00001);
    let c1 = computer.get_state().vec.get(1).unwrap();
    assert_approx_eq!(f64, c1.re(), 1.0, epsilon = 0.00001);
    assert_approx_eq!(f64, c1.im(), 0.0, epsilon = 0.00001);
}

#[test]
fn pauli_y_1_qbit() {
    let mut computer = QuantumComputer::new(1, None);
    PauliY::new(0).apply(&mut computer);
    let c0 = computer.get_state().vec.get(0).unwrap();
    assert_approx_eq!(f64, c0.re(), 0.0, epsilon = 0.00001);
    assert_approx_eq!(f64, c0.im(), 0.0, epsilon = 0.00001);
    let c1 = computer.get_state().vec.get(1).unwrap();
    assert_approx_eq!(f64, c1.re(), 0.0, epsilon = 0.00001);
    assert_approx_eq!(f64, c1.im(), 1.0, epsilon = 0.00001);
}

#[test]
fn pauli_z_1_qbit() {
    let mut computer = QuantumComputer::new(1, None);
    PauliX::new(0).apply(&mut computer);
    PauliZ::new(0).apply(&mut computer);
    let c0 = computer.get_state().vec.get(0).unwrap();
    approx_eq!(f64, c0.re(), 0.0, epsilon = 0.00001);
    approx_eq!(f64, c0.im(), 0.0, epsilon = 0.00001);
    let c1 = computer.get_state().vec.get(1).unwrap();
    assert_approx_eq!(f64, c1.re(), -1.0, epsilon = 0.00001);
    assert_approx_eq!(f64, c1.im(), 0.0, epsilon = 0.00001);
}

#[test]
fn hadamard_1_qbit() {
    let mut computer = QuantumComputer::new(1, None);
    Hadamard::new(0).apply(&mut computer);
    for amplitude in computer.get_state().amplitudes() {
        assert_approx_eq!(f64, amplitude, 0.5, epsilon = 0.00001);
    }
}

#[test]
fn hadamard_3_qbit() {
    let mut computer = QuantumComputer::new(3, None);
    Hadamard::new(0).apply(&mut computer);
    Hadamard::new(2).apply(&mut computer);
    for (i, &amplitude) in computer.get_state().amplitudes().iter().enumerate() {
        if i == 0b000 || i == 0b001 || i == 0b100 || i == 0b101 {
            assert_approx_eq!(f64, amplitude, 0.25, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, amplitude, 0.0, epsilon = 0.00001);
        }
    }
}

#[test]
fn bell_3_qbit() {
    let mut computer = QuantumComputer::new(3, None);
    Hadamard::new(2).apply(&mut computer);
    Controlled::<2, _>::new(2, PauliX::new(0)).apply(&mut computer);
    for (i, &amplitude) in computer.get_state().amplitudes().iter().enumerate() {
        if i == 0b000 || i == 0b101 {
            assert_approx_eq!(f64, amplitude, 0.5, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, amplitude, 0.0, epsilon = 0.00001);
        }
    }
}

#[test]
fn bell_5_qbit() {
    let mut computer = QuantumComputer::new(5, None);
    Hadamard::new(2).apply(&mut computer);
    Controlled::<2, _>::new(2, PauliX::new(0)).apply(&mut computer);
    Controlled::<2, _>::new(0, PauliX::new(4)).apply(&mut computer);
    for (i, &amplitude) in computer.get_state().amplitudes().iter().enumerate() {
        if i == 0b00000 || i == 0b10101 {
            assert_approx_eq!(f64, amplitude, 0.5, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, amplitude, 0.0, epsilon = 0.00001);
        }
    }
}

#[test]
fn ptest() {
    let h = Hadamard::new(2);
    let mut computer = QuantumComputer::new(2, None);
    h.to_gate().apply(&mut computer, [0]);
    h.to_gate().apply(&mut computer, [1]);
    let state = computer.state().clone();

    let n = 2048; // Simulation Steps
    let mut computer = QuantumComputer::new(2, None);
    let rrh: Gate<1> = h.parameterized()(&h, PI / (n as f64));
    let hh: Gate<2> = rrh ^ rrh;
    for _ in 0..n {
        hh.apply(&mut computer, [0, 1]);
    }
    let state0 = computer.state().clone();

    let mut computer = QuantumComputer::new(2, None);
    for _ in 0..n {
        rrh.apply(&mut computer, [0]);
        rrh.apply(&mut computer, [1]);
    }
    let state1 = computer.state().clone();

    for (c0, c1) in state.iter().zip(state0) {
        assert_approx_eq!(Complex, *c0, c1, epsilon = 0.00001);
    }
    for (c0, c1) in state.iter().zip(state1) {
        assert_approx_eq!(Complex, *c0, c1, epsilon = 0.00001);
    }
}

#[test]
fn bv_small() {
    let m = [
        true, true, false, false, false
    ];
    let mut computer = QuantumComputer::new(m.len() + 1, None);
    for i in 0..m.len() + 1 {
        Hadamard::new(i).apply(&mut computer);
    }
    PauliZ::new(m.len()).apply(&mut computer);
    for i in 0..m.len() {
        if m[i] == true {
            Controlled::<2, _>::new(i, PauliX::new(m.len())).apply(&mut computer);
        }
    }
    for i in 0..m.len() {
        Hadamard::new(i).apply(&mut computer);
    }
    let probabilities = computer.probabilities();
    for (&p, v) in probabilities.iter().zip(m) {
        if v {
            assert_approx_eq!(f64, p, 1.0, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, p, 0.0, epsilon = 0.00001);
        }
    }
}

#[test]
fn bv_medium() {
    let m = [
        true, true, false, true, false, true, true, false,
        true, false, true, true, false, true, false,
    ];
    let mut computer = QuantumComputer::new(m.len() + 1, None);
    for i in 0..m.len() + 1 {
        Hadamard::new(i).apply(&mut computer);
    }
    PauliZ::new(m.len()).apply(&mut computer);
    for i in 0..m.len() {
        if m[i] == true {
            Controlled::<2, _>::new(i, PauliX::new(m.len())).apply(&mut computer);
        }
    }
    for i in 0..m.len() {
        Hadamard::new(i).apply(&mut computer);
    }
    let probabilities = computer.probabilities();
    for (&p, v) in probabilities.iter().zip(m) {
        if v {
            assert_approx_eq!(f64, p, 1.0, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, p, 0.0, epsilon = 0.00001);
        }
    }
}

#[test]
#[ignore]
fn bv_large() {
    let m = [
        true, true, false, true, false, true, true, false, true, false, true, true,
        false, true, false, true, true, false, true, false, true, true, false, true,
        false, true, true,
    ];
    let mut computer = QuantumComputer::new(m.len() + 1, None);
    for i in 0..m.len() + 1 {
        Hadamard::new(i).apply(&mut computer);
    }
    PauliZ::new(m.len()).apply(&mut computer);
    for i in 0..m.len() {
        if m[i] == true {
            Controlled::<2, _>::new(i, PauliX::new(m.len())).apply(&mut computer);
        }
    }
    for i in 0..m.len() {
        Hadamard::new(i).apply(&mut computer);
    }
    let probabilities = computer.probabilities();
    for (&p, v) in probabilities.iter().zip(m) {
        if v {
            assert_approx_eq!(f64, p, 1.0, epsilon = 0.00001);
        } else {
            assert_approx_eq!(f64, p, 0.0, epsilon = 0.00001);
        }
    }
}

#[bench]
fn bv_bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let m = [
            true, true, false, true, false, true, true,
            false, true, false, true, false
        ];
        let mut computer = QuantumComputer::new(m.len() + 1, None);
        for i in 0..m.len() + 1 {
            Hadamard::new(i).apply(&mut computer);
        }
        PauliZ::new(m.len()).apply(&mut computer);
        for i in 0..m.len() {
            if m[i] == true {
                Controlled::<2, _>::new(i, PauliX::new(m.len())).apply(&mut computer);
            }
        }
        for i in 0..m.len() {
            Hadamard::new(i).apply(&mut computer);
        }

        computer
    });
}
