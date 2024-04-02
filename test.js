import * as demo from './pkg/demo.js';
import * as fs from 'node:fs';
import { performance } from 'node:perf_hooks';
import test from 'node:test';
import { strict as assert } from 'node:assert';

const m = fs.readFileSync('./pkg/demo_bg.wasm');
demo.initSync(m);

test('outputs match', () => {
    const a = demo.demo();
    const b = demo.demo2();
    fs.writeFileSync("data.json", JSON.stringify(b));
    assert.deepStrictEqual(a, b);
});

function bench(desc, fn) {
    const start = performance.now();
    const iters = 1000;
    for (let i = 0; i < iters; i++) {
        fn();
    }
    const end = performance.now();

    const delta = end - start;
    console.log(`${desc}: ${iters} iterations in ${delta.toFixed(0)}ms = ${(delta / iters).toFixed(1)}ms per`);
    return delta;
}

test('performance', () => {
    const wasm_serialize = bench('wasm-serialize', demo.demo);
    const serde_wasm_bindgen = bench('serde-wasm-bindgen', demo.demo2);
    const frac = wasm_serialize / serde_wasm_bindgen;
    console.log(`wasm-serialize ${(frac * 100).toFixed(1)}% of serde-wasm-bindgen`);
});
