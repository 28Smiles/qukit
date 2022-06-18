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
