import {GateBuilder, hadamard, pauliZ, cPauliX, QBit, Bit, measurement} from "../../pkg.nodejs";

function bvAlgorithm(hidden: boolean[]) {
    const builder = new GateBuilder();
    const qbits: QBit[] = builder.qbits(hidden.length);
    const bits: Bit[] = builder.bits(hidden.length);
    const target: QBit = builder.qbit();

    hadamard(target);
    pauliZ(target);

    hadamard(qbits);

    hidden.forEach((active, index) => {
        if (active) {
            cPauliX(qbits[index], target);
        }
    });

    hadamard(qbits);
    hadamard(target);

    measurement(qbits, bits);

    const result = builder.intoAlgorithm().run();
    const measurements = result.classicalRegister().state();

    hidden.forEach((hidden, index) => {
        expect(hidden).toBe(measurements[index]);
    });
}

test('test extreme bv', () => {
    bvAlgorithm([
        true, true, false, true, false,
        true, true, false,  true, true,
        false, true, false, true, true,
        false, true, false, true, true,
        false, true, false, false, true
    ]);
});

test('test large bv', () => {
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
