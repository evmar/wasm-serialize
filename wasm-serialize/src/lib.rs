use wasm_bindgen::prelude::*;

pub trait WasmSerialize {
    type JsType: ?Sized;
    fn to_wasm(&self) -> JsValue;
}

impl WasmSerialize for u32 {
    type JsType = u32;
    fn to_wasm(&self) -> JsValue {
        JsValue::from(*self)
    }
}

impl WasmSerialize for String {
    type JsType = String;
    fn to_wasm(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl WasmSerialize for &str {
    type JsType = str;
    fn to_wasm(&self) -> JsValue {
        JsValue::from_str(self)
    }
}

impl<T: WasmSerialize> WasmSerialize for [T] {
    type JsType = [T];
    fn to_wasm(&self) -> JsValue {
        let arr = js_sys::Array::new();
        for val in self.iter() {
            arr.push(&val.to_wasm());
        }
        arr.into()
    }
}

impl<T: WasmSerialize> WasmSerialize for Vec<T> {
    type JsType = <[T] as WasmSerialize>::JsType;
    fn to_wasm(&self) -> JsValue {
        self.as_slice().to_wasm()
    }
}
