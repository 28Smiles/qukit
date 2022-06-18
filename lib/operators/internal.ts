import {
    Hadamard,
    PauliX,
    PauliXRoot,
    PauliY,
    PauliZ, RotationHadamard, RotationPauliX, RotationPauliY, RotationPauliZ, RotationSwap,
    RotationX, RotationY, RotationZ,
    SGate,
    SGateInverse,
    Swap,
    SwapRoot,
    TGate
} from "@/pkg/qukit";

export class OneWire implements PauliX, PauliY, PauliZ, Hadamard, TGate, SGate, SGateInverse, PauliXRoot {
    wire: number;

    constructor(wire: number) {
        this.wire = wire;
    }
}

export class TwoWire implements Swap, SwapRoot {
    wire0: number;
    wire1: number;

    constructor(wire0: number, wire1: number) {
        this.wire0 = wire0;
        this.wire1 = wire1;
    }
}

export class RotationOneWire extends OneWire implements RotationX, RotationY, RotationZ, RotationPauliX, RotationPauliY, RotationPauliZ, RotationHadamard {
    theta: number;
    constructor(wire: number, theta: number) {
        super(wire);
        this.theta = theta;
    }
}

export class RotationTwoWire extends TwoWire implements RotationSwap {
    theta: number;
    constructor(wire0: number, wire1: number, theta: number) {
        super(wire0, wire1);
        this.theta = theta;
    }
}
