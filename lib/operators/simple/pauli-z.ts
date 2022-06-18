import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class PauliZ extends OneWire implements WasmLib.Simple.PauliZ {
    type: "PauliZ" = "PauliZ";
}
