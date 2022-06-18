import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationPauliZ extends RotationOneWire implements WasmLib.Rotation.RotationPauliZ {
    type: "RotationPauliZ" = "RotationPauliZ";
}
