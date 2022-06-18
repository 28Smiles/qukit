import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationY extends RotationOneWire implements WasmLib.Rotation.RotationY {
    type: "RotationY" = "RotationY";
}
