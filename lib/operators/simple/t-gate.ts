import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class TGate extends OneWire implements WasmLib.Simple.TGate {
    type: "TGate" = "TGate";
}
