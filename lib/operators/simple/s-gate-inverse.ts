import {WasmLib} from "@/index";
import {OneWire} from "@/operators/internal";

export class SGateInverse extends OneWire implements WasmLib.Simple.SGateInverse {
    type: "SGateInverse" = "SGateInverse";
}
