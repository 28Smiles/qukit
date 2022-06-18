import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class PauliX extends OneWire implements WasmLib.Simple.PauliX {
    type: "PauliX" = "PauliX";
}
