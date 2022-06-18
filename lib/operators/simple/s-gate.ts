import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class SGate extends OneWire implements WasmLib.Simple.SGate {
    type: "SGate" = "SGate";
}
