import {WasmLib} from "@/index";
import {RotationOneWire} from "@/operators/internal";

export class RotationHadamard extends RotationOneWire implements WasmLib.Rotation.RotationHadamard {
    type: "RotationHadamard" = "RotationHadamard";
}
