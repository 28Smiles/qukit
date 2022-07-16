use qukit::api::derive::{Algorithm, controlled_pauli_x, hadamard, measurement_z, pauli_z};

fn main() {
    let hidden = Vec::from([
        true, true, false, true, false,
    ]);
    let algorithm = Algorithm::new(|gate_builder| {
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

    println!("{}", quantum_register);
    for (hidden, measured) in hidden.iter().zip(classical_register.state().iter()) {
        assert_eq!(*hidden, *measured);
    }
}
