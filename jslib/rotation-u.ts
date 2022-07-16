import { QBit, Bit } from "@/pkg/qukit";
import * as WasmLib from "@/pkg/qukit";

export function rotationU< Q extends QBit | QBit[] >(theta: number, lambda: number, phi: number, qbits: Q, cControl?: Bit, sameStep: boolean = false): void {
    if (Array.isArray(qbits)) {
        const minLen = Math.min(qbits.length);
        if (minLen > 0) {
            rotationU(theta, lambda, phi, qbits[0], cControl, sameStep);
        }
        for (let i = 1; i < minLen; i++) {
            rotationU(theta, lambda, phi, qbits[i], cControl, sameStep);
        }
    } else {
        if (!Array.isArray(qbits)) {
            if (cControl !== null && cControl !== undefined) {
                if (sameStep) {
                    WasmLib.rotation_u_same_step_classically_controlled(theta, lambda, phi, qbits, cControl);
                } else {
                    WasmLib.rotation_u_classically_controlled(theta, lambda, phi, qbits, cControl);
                }
            } else {
                if (sameStep) {
                    WasmLib.rotation_u_same_step(theta, lambda, phi, qbits);
                } else {
                    WasmLib.rotation_u(theta, lambda, phi, qbits);
                }
            }
        } else {
            throw new Error("type mismatch");
        }
    }
}
