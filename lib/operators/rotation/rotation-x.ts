import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationX extends RotationOneWire implements WasmLib.Rotation.RotationX {
    type: "RotationX" = "RotationX";
}
