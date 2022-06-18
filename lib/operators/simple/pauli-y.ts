import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class PauliY extends OneWire implements WasmLib.Simple.PauliY {
    type: "PauliY" = "PauliY";
}
