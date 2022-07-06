const fs = require("fs");

const imports = [
    [ "hadamard", 1, false ],
    [ "pauli_x", 1, false ],
    [ "pauli_y", 1, false ],
    [ "pauli_z", 1, false ],
    [ "phase", 1, false ],
    [ "phase_dagger", 1, false ],
    [ "phase_root", 1, false ],
    [ "phase_root_dagger", 1, false ],
    [ "pauli_x_root", 1, false ],
    [ "swap", 2, false ],
    [ "swap_root", 2, false ],

    [ "rotation_hadamard", 1, true ],
    [ "rotation_pauli_x", 1, true ],
    [ "rotation_pauli_y", 1, true ],
    [ "rotation_pauli_z", 1, true ],
    [ "rotation_x", 1, true ],
    [ "rotation_y", 1, true ],
    [ "rotation_z", 1, true ],
    [ "rotation_swap", 2, true ],
].flatMap(([name, size, rotation ]) => {
    const functionName = name.replaceAll(/_([a-z])/g, (v) => v.slice(1).toUpperCase());
    const fileName = name.replaceAll("_", "-");
    const rotationArg = rotation ? "theta: number, " : "";
    const rotationCallArg = rotation ? "theta, " : "";
    const args = size > 1 ? [...Array(size).keys()].map(idx => `qbits${idx}: Q`).join(", ") : `qbits: Q`;
    const callArgs = size > 1 ? (i) => {
        if (i === undefined) {
            return [...Array(size).keys()].map(idx => `qbits${idx}`);
        } else {
            return [...Array(size).keys()].map(idx => `qbits${idx}[${i}]`);
        }
    } : (i) => {
        if (i === undefined) {
            return [`qbits`];
        } else {
            return [`qbits[${i}]`];
        }
    };

    return [ 0, 1, 2 ].map(controls => {
        const cName = [...[...Array(controls).keys()].map(_ => `controlled`), name].join("_");
        const cFileName = [...[...Array(controls).keys()].map(_ => `controlled`), fileName].join("-");
        const cFunctionName = controls === 0 ?
            functionName
            :`${[...Array(controls).keys()].map(_ => `c`).join("")}${functionName.slice(0, 1).toUpperCase()}${functionName.slice(1)}`;
        const cArgs = [...(controls === 1 ? [`cQbits: Q`] : [...Array(controls).keys()].map(idx => `cQbits${idx}: Q`)), args].join(", ");
        const cCallArgs = controls !== 1 ? (i) => {
            if (i === undefined) {
                return [...[...Array(controls).keys()].map(idx => `cQbits${idx}`), ...callArgs(i)];
            } else {
                return [...[...Array(controls).keys()].map(idx => `cQbits${idx}[${i}]`), ...callArgs(i)];
            }
        } : (i) => {
            if (i === undefined) {
                return [`cQbits`, ...callArgs(i)];
            } else {
                return [`cQbits[${i}]`, ...callArgs(i)];
            }
        };
        fs.writeFileSync(`./jslib/gates/${cFileName}.ts`,
            `import { QBit, Bit } from "@/pkg/qukit";
import * as WasmLib from "@/pkg/qukit";

export function ${cFunctionName}< Q extends QBit | QBit[] >(${rotationArg}${cArgs}${controls === 0 ? ", cControl?: Bit" : ""}, sameStep: boolean = false): void {
    if (${cCallArgs().map(arg => `Array.isArray(${arg})`).join(" && ")}) {
        const minLen = Math.min(${cCallArgs().map(arg => `${arg}.length`).join(", ")});
        if (minLen > 0) {
            ${cFunctionName}(${rotationCallArg}${cCallArgs("0").join(", ")}${controls === 0 ? ", cControl" : ""}, sameStep);
        }
        for (let i = 1; i < minLen; i++) {
            ${cFunctionName}(${rotationCallArg}${cCallArgs("i").join(", ")}${controls === 0 ? ", cControl" : ""}, sameStep);
        }
    } else {
        if (${cCallArgs().map(arg => `!Array.isArray(${arg})`).join(" && ")}) {${controls === 0 ? `
            if (cControl !== null && cControl !== undefined) {
                if (sameStep) {
                    WasmLib.${cName}_same_step_classically_controlled(${rotationCallArg}${cCallArgs().join(", ")}, cControl);
                } else {
                    WasmLib.${cName}_classically_controlled(${rotationCallArg}${cCallArgs().join(", ")}, cControl);
                }
            } else {`: ""}
                if (sameStep) {
                    WasmLib.${cName}_same_step(${rotationCallArg}${cCallArgs().join(", ")});
                } else {
                    WasmLib.${cName}(${rotationCallArg}${cCallArgs().join(", ")});
                }${controls === 0 ? `
            }` : ""}
        } else {
            throw new Error("type mismatch");
        }
    }
}`
        );

        return `export {${cFunctionName}} from "./gates/${cFileName}";\n`;
    })
});

fs.writeFileSync("jslib/gates.ts", imports.join(""));
