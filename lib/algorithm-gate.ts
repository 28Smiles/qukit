import { WasmLib } from ".";

export class AlgorithmGate implements WasmLib.AlgorithmGate {
    gate: WasmLib.OperatorType;
    position: number;

    constructor(gate: WasmLib.OperatorType, position: number) {
        this.gate = gate;
        this.position = position;
    }
}
