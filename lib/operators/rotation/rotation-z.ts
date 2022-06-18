import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationZ extends RotationOneWire implements WasmLib.Rotation.RotationZ {
    type: "RotationZ" = "RotationZ";
}
