mod serde;

pub use self::serde::{FromFfi, ToFfi};
pub use wasm_bindgen::prelude::*;

pub fn map_err(msg: String) -> JsError {
    JsError::new(&msg)
}
