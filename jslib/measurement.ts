import { QBit, Bit } from "@/pkg/qukit";
import * as WasmLib from "@/pkg/qukit";

export function measurement(qbits: QBit | QBit[], cbits: Bit | Bit[], basis: "x" | "y" | "z" = "z", sameStep: boolean = false): void {
    if (Array.isArray(qbits) && Array.isArray(cbits)) {
        const minLen = Math.min(qbits.length, cbits.length);
        if (minLen > 0) {
            measurement(qbits[0], cbits[0], basis, sameStep);
        }
        for (let i = 1; i < minLen; i++) {
            measurement(qbits[i], cbits[i], basis, sameStep);
        }
    } else {
        if (!Array.isArray(qbits) && !Array.isArray(cbits)) {
            if (sameStep) {
                switch (basis) {
                    case "x":
                        WasmLib.measurement_x_same_step(qbits, cbits);
                        break;
                    case "y":
                        WasmLib.measurement_y_same_step(qbits, cbits);
                        break;
                    case "z":
                        WasmLib.measurement_z_same_step(qbits, cbits);
                        break;
                }
            } else {
                switch (basis) {
                    case "x":
                        WasmLib.measurement_x(qbits, cbits);
                        break;
                    case "y":
                        WasmLib.measurement_y(qbits, cbits);
                        break;
                    case "z":
                        WasmLib.measurement_z(qbits, cbits);
                        break;
                }
            }
        } else {
            throw new Error("type mismatch");
        }
    }
}
