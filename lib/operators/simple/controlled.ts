import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class Controlled<T extends WasmLib.Simple | WasmLib.Rotation> extends OneWire implements WasmLib.Controlled<T> {
    type: `Controlled${T["type"]}`;
    transformation: T;

    constructor(wire: number, transformation: T) {
        super(wire);
        this.transformation = transformation;
        this.type = `Controlled${transformation.type}` as `Controlled${T["type"]}`;
    }
}
