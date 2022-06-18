import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationPauliY extends RotationOneWire implements WasmLib.Rotation.RotationPauliY {
    type: "RotationPauliY" = "RotationPauliY";
}
