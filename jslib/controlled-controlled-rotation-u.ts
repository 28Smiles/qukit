import { QBit, Bit } from "@/pkg/qukit";
import * as WasmLib from "@/pkg/qukit";

export function ccRotationU< Q extends QBit | QBit[] >(theta: number, lambda: number, phi: number, cQbits0: Q, cQbits1: Q, qbits: Q, sameStep: boolean = false): void {
    if (Array.isArray(cQbits0) && Array.isArray(cQbits1) && Array.isArray(qbits)) {
        const minLen = Math.min(cQbits0.length, cQbits1.length, qbits.length);
        if (minLen > 0) {
            ccRotationU(theta, lambda, phi, cQbits0[0], cQbits1[0], qbits[0], sameStep);
        }
        for (let i = 1; i < minLen; i++) {
            ccRotationU(theta, lambda, phi, cQbits0[i], cQbits1[i], qbits[i], sameStep);
        }
    } else {
        if (!Array.isArray(cQbits0) && !Array.isArray(cQbits1) && !Array.isArray(qbits)) {
            if (sameStep) {
                WasmLib.controlled_controlled_rotation_u_same_step(theta, lambda, phi, cQbits0, cQbits1, qbits);
            } else {
                WasmLib.controlled_controlled_rotation_u(theta, lambda, phi, cQbits0, cQbits1, qbits);
            }
        } else {
            throw new Error("type mismatch");
        }
    }
}
