import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class PauliXRoot extends OneWire implements WasmLib.Simple.PauliXRoot {
    type: "PauliXRoot" = "PauliXRoot";
}
