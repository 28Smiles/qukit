import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class Hadamard extends OneWire implements WasmLib.Simple.Hadamard {
    type: "Hadamard" = "Hadamard";
}
