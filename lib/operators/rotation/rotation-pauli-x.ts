import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationPauliX extends RotationOneWire implements WasmLib.Rotation.RotationPauliX {
    type: "RotationPauliX" = "RotationPauliX";
}
