import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class Measure extends OneWire implements WasmLib.Special.Measure {
    type: "Measure" = "Measure";
    basis?: WasmLib.MeasurementBasis;
    creg?: number;
    cregBit?: number;

    constructor(wire: number, basis?: WasmLib.MeasurementBasis, creg?: number, cregBit?: number) {
        super(wire);
        this.basis = basis;
        this.creg = creg;
        this.cregBit = cregBit;
    }
}
