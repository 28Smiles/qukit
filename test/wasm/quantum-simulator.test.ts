import {controlled_pauli_x, GateBuilder, hadamard, measurement_z, pauli_z} from "../../pkg";

function bvAlgorithm(hidden: boolean[]) {
    const builder = new GateBuilder();
    const qbits = builder.qbits(hidden.length);
    const bits = builder.bits(hidden.length);
    const target = builder.qbit();

    hadamard(target);
    pauli_z(target);

    qbits.forEach((qbit) => hadamard(qbit));

    hidden.forEach((active, index) => {
        if (active) {
            controlled_pauli_x(qbits[index], target);
        }
    });

    qbits.forEach((qbit) => hadamard(qbit));
    hadamard(target);

    qbits.forEach((qbit, index) => measurement_z(qbit, bits[index]));

    const result = builder.into_algorithm().run();
    const measurements = result.classical_register().state();

    hidden.forEach((hidden, index) => {
        expect(hidden).toBe(measurements[index]);
    });
}

test('test large', () => {
    bvAlgorithm([
        true, true, false, true, false,
        true, true, false,  true, true,
        false, true, false, true, true,
        false, true, false, true, true,
        false, true, false, false, true
    ]);
});

test('test med bv', () => {
    bvAlgorithm([
        true, true, false, true, false,
        true, true, false, false, true,
        false, true
    ]);
});

test('test med bv', () => {
    bvAlgorithm([
        true, true, false, true, false, true, true, false,
    ]);
});

test('test small bv', () => {
    bvAlgorithm([
        true, true, false, true, false,
    ]);
});
