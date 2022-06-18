import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class Reset extends OneWire implements WasmLib.Special.Reset {
    type: "Reset" = "Reset";
    state: boolean;

    constructor(wire: number, state: boolean) {
        super(wire);
        this.state = state;
    }
}
