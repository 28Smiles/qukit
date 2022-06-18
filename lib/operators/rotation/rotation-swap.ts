import {WasmLib} from "@/index";
import {RotationTwoWire} from "@/operators/internal";

export class RotationSwap extends RotationTwoWire implements WasmLib.Rotation.RotationSwap {
    type: "RotationSwap" = "RotationSwap";
}
