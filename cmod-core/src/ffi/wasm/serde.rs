use serde::{Deserialize, Serialize};
use wasm_bindgen::convert::*;
use wasm_bindgen::describe::*;
use wasm_bindgen::prelude::*;

/// 从外部语言转为Rust类型
pub struct FromFfi<T>(T);
impl<T> FromFfi<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T> WasmDescribe for FromFfi<T> {
    fn describe() {
        JsValue::describe();
    }
}
impl<T: for<'de> Deserialize<'de>> FromWasmAbi for FromFfi<T> {
    type Abi = u32;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        JsValue::from_abi(js).into()
    }
}
impl<T: for<'de> Deserialize<'de>> OptionFromWasmAbi for FromFfi<T> {
    fn is_none(abi: &Self::Abi) -> bool {
        abi == &JsValue::UNDEFINED.into_abi() || abi == &JsValue::NULL.into_abi()
    }
}
impl<T: for<'de> Deserialize<'de>> From<JsValue> for FromFfi<T> {
    fn from(value: JsValue) -> Self {
        Self(serde_wasm_bindgen::from_value(value).unwrap())
    }
}

/// 从Rust类型转为外部语言
pub struct ToFfi<T>(T);
impl<T> From<T> for ToFfi<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
impl<T> WasmDescribe for ToFfi<T> {
    fn describe() {
        JsValue::describe();
    }
}
impl<T: Serialize> IntoWasmAbi for ToFfi<T> {
    type Abi = u32;

    fn into_abi(self) -> Self::Abi {
        JsValue::from(self).into_abi()
    }
}
impl<T: Serialize> OptionIntoWasmAbi for ToFfi<T> {
    fn none() -> Self::Abi {
        JsValue::NULL.into_abi()
    }
}
impl<T: Serialize> From<ToFfi<T>> for JsValue {
    fn from(value: ToFfi<T>) -> Self {
        serde_wasm_bindgen::to_value(&value.0).unwrap()
    }
}
