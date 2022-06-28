const {QuantumAlgorithm, QuantumComputer} = require("../../pkg/qukit");

test('test hadamard', () => {
    const computer = new QuantumComputer(2);
    const algorithm = new QuantumAlgorithm([
        { position: 0, gate: { type: "Hadamard", wire: 0 } },
        { position: 0, gate: { type: "Hadamard", wire: 1 } },
    ]);
    algorithm.apply(computer);
    const state0 = computer.state();
    expect(state0[0].re).toBeCloseTo(0.5);
    expect(state0[1].re).toBeCloseTo(0.5);
    expect(state0[2].re).toBeCloseTo(0.5);
    expect(state0[3].re).toBeCloseTo(0.5);
    expect(state0[0].im).toBeCloseTo(0);
    expect(state0[1].im).toBeCloseTo(0);
    expect(state0[2].im).toBeCloseTo(0);
    expect(state0[3].im).toBeCloseTo(0);
    algorithm.apply(computer);
    const state1 = computer.state();
    expect(state1[0].re).toBeCloseTo(0.5);
    expect(state1[1].re).toBeCloseTo(0.5);
    expect(state1[2].re).toBeCloseTo(0.5);
    expect(state1[3].re).toBeCloseTo(0.5);
    expect(state1[0].im).toBeCloseTo(0);
    expect(state1[1].im).toBeCloseTo(0);
    expect(state1[2].im).toBeCloseTo(0);
    expect(state1[3].im).toBeCloseTo(0);
});

test('test bell', () => {
    const computer = new QuantumComputer(2);
    const algorithm = new QuantumAlgorithm([
        { position: 0, gate: { type: "Hadamard", wire: 0 } },
        { position: 1, gate: { type: "ControlledPauliX", wire: 0, transformation: { wire: 1 } } },
    ]);
    algorithm.apply(computer);
    const state0 = computer.state();
    expect(state0[0].re).toBeCloseTo(Math.SQRT1_2);
    expect(state0[1].re).toBeCloseTo(Math.SQRT1_2);
    expect(state0[2].re).toBeCloseTo(0);
    expect(state0[3].re).toBeCloseTo(0);
    expect(state0[0].im).toBeCloseTo(0);
    expect(state0[1].im).toBeCloseTo(0);
    expect(state0[2].im).toBeCloseTo(0);
    expect(state0[3].im).toBeCloseTo(0);
    algorithm.apply(computer);
    const state1 = computer.state();
    expect(state1[0].re).toBeCloseTo(Math.SQRT1_2);
    expect(state1[1].re).toBeCloseTo(0);
    expect(state1[2].re).toBeCloseTo(0);
    expect(state1[3].re).toBeCloseTo(Math.SQRT1_2);
    expect(state1[0].im).toBeCloseTo(0);
    expect(state1[1].im).toBeCloseTo(0);
    expect(state1[2].im).toBeCloseTo(0);
    expect(state1[3].im).toBeCloseTo(0);
});

test('test steps', () => {
    const computer = new QuantumComputer(1);
    const algorithm = new QuantumAlgorithm([
        { position: 0, gate: { type: "Hadamard", wire: 0 } },
    ], 8);

    for (let i = 0; i < 7; i++) {
        expect(algorithm.apply(computer)).toBe(true);
        const state = computer.state();
        expect(state[0].re).not.toBeCloseTo(Math.SQRT1_2);
        expect(state[1].re).not.toBeCloseTo(Math.SQRT1_2);
    }
    expect(algorithm.apply(computer)).toBe(true);
    const state0 = computer.state();
    expect(state0[0].re).toBeCloseTo(Math.SQRT1_2);
    expect(state0[1].re).toBeCloseTo(Math.SQRT1_2);
    expect(state0[0].im).toBeCloseTo(0);
    expect(state0[1].im).toBeCloseTo(0);
    expect(algorithm.apply(computer)).toBe(false);
    const state1 = computer.state();
    expect(state1[0].re).toBeCloseTo(Math.SQRT1_2);
    expect(state1[1].re).toBeCloseTo(Math.SQRT1_2);
    expect(state1[0].im).toBeCloseTo(0);
    expect(state1[1].im).toBeCloseTo(0);
});

test('test error', () => {
    expect(() => new QuantumAlgorithm([
        {type: "ch", wire: 0},
    ], 8)).toThrowError();
});

function bvAlgorithm(hidden: boolean[]) {
    const computer = new QuantumComputer(hidden.length + 1);
    const algorithm = new QuantumAlgorithm([
        ...hidden.map((_, index) => ({ type: "Hadamard", wire: index })),
        { type: "Hadamard", wire: hidden.length },
        { type: "PauliZ", wire: hidden.length },

        ...hidden.map((v, index) =>
            v ? {
                type: "ControlledPauliX",
                wire: index,
                transformation: {
                    type: "PauliX",
                    wire: hidden.length,
                },
            } : undefined
        ).filter(value => value !== undefined),

        ...hidden.map((_, index) => ({ type: "Hadamard", wire: index })),
        //...hidden.split("").map((_, index) => new Measurement(index, "X")),
    ].map((v, index) => ({
        position: index,
        gate: v,
    })), 1);
    algorithm.run(computer);
    const state = computer.probabilities();
    for (let index = 0; index < hidden.length; index++) {
        const exp = hidden[index];
        const prob = state[index];
        if (exp) {
            expect(prob).toBeCloseTo(1, 0.00001);
        } else {
            expect(prob).toBeCloseTo(0, 0.00001);
        }
    }
}

test.skip('test large', () => {
    bvAlgorithm([
        true, true, false, true, false, true, true, false,
        true, false, true, true, false, true, false
    ]);
});

test('test med bv', () => {
    bvAlgorithm([
        true, true, false, true, false, true, true, false,
        true, false, true, true, false, true, false
    ]);
});

test('test small bv', () => {
    bvAlgorithm([
        true, true, false, true, false, true, true, false,
        true, false, true, true, false, true, false
    ]);
});
