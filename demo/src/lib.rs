use wasm_bindgen::prelude::*;
use wasm_serialize::WasmSerialize;
use wasm_serialize_derive::WasmSerialize;

#[derive(serde::Serialize, serde::Deserialize, WasmSerialize)]
pub struct CodePart {
    pub kind: String,
    pub text: String,
}

#[derive(serde::Serialize, serde::Deserialize, WasmSerialize)]
pub struct Instruction {
    pub addr: u32,
    pub bytes: String,
    pub code: Vec<CodePart>,
    pub ops: Vec<String>,
}

#[derive(WasmSerialize)]
pub struct Instrs(Vec<Instruction>);

// XXX cannot return a Vec directly using to_wasm because wasm-bindgen implements IntoWasmAbi.
#[wasm_bindgen]
pub fn demo() -> Instrs {
    let instrs = testdata();
    Instrs(instrs)
}

#[wasm_bindgen]
pub fn demo2() -> JsValue {
    let instrs = testdata();
    serde_wasm_bindgen::to_value(&instrs).unwrap()
}

// TODO: don't include data deserialization in measurement.
const DATA_JSON: &[u8] = include_bytes!("data.json");
fn testdata() -> Vec<Instruction> {
    serde_json::from_slice(DATA_JSON).unwrap()
}
