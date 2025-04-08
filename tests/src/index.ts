import * as wasm from "../pkg/crypto_wasm";

const result = wasm.eddsa_keygen();
console.log("Result from WASM:", result);
