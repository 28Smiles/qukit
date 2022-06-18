import {WasmLib} from "@/index";
import {TwoWire} from "@/operators/internal";

export class Swap extends TwoWire implements WasmLib.Simple.Swap {
    type: "Swap" = "Swap";
}
