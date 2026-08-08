import { readFileSync } from "node:fs";

async function load(name) {
	const path = new URL(`target/${name}/wasm32-unknown-unknown/release/urandom_wasm_tests.wasm`, import.meta.url);
	const bytes = readFileSync(path);
	const { instance } = await WebAssembly.instantiate(bytes, {});
	return instance.exports;
}

const scalar = await load("scalar");
const simd = await load("simd");
const scalarFingerprint = scalar.fingerprint() >>> 0;
const simdFingerprint = simd.fingerprint() >>> 0;

if (scalar.verify() !== 1) throw new Error(`scalar: unexpected fingerprint ${scalarFingerprint}`);
if (simd.verify() !== 1) throw new Error(`simd: unexpected fingerprint ${simdFingerprint}`);
if (simdFingerprint !== scalarFingerprint) {
	throw new Error(`SIMD fingerprint ${simdFingerprint} differs from scalar ${scalarFingerprint}`);
}

console.log(`scalar and SIMD128 checks passed; fingerprint=${scalarFingerprint}`);
