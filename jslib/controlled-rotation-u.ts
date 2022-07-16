import { QBit, Bit } from "@/pkg/qukit";
import * as WasmLib from "@/pkg/qukit";

export function cRotationU< Q extends QBit | QBit[] >(theta: number, lambda: number, phi: number, cQbits: Q, qbits: Q, sameStep: boolean = false): void {
    if (Array.isArray(cQbits) && Array.isArray(qbits)) {
        const minLen = Math.min(cQbits.length, qbits.length);
        if (minLen > 0) {
            cRotationU(theta, lambda, phi, cQbits[0], qbits[0], sameStep);
        }
        for (let i = 1; i < minLen; i++) {
            cRotationU(theta, lambda, phi, cQbits[i], qbits[i], sameStep);
        }
    } else {
        if (!Array.isArray(cQbits) && !Array.isArray(qbits)) {
            if (sameStep) {
                WasmLib.controlled_rotation_u_same_step(theta, lambda, phi, cQbits, qbits);
            } else {
                WasmLib.controlled_rotation_u(theta, lambda, phi, cQbits, qbits);
            }
        } else {
            throw new Error("type mismatch");
        }
    }
}
