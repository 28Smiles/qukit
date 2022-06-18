import {WasmLib} from "@/index";
import {TwoWire} from "@/operators/internal";

export class SwapRoot extends TwoWire implements WasmLib.Simple.SwapRoot {
    type: "SwapRoot" = "SwapRoot";
}
