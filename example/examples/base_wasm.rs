use wasm_bindgen::prelude::*;
pub mod hello {
    use crate::A;
    use cmod::Result;
    use wasm_bindgen::prelude;

    #[wasm_bindgen(js_name = hello_world)]
    pub fn hello_world() -> Result<cmod::ffi::wasm::ToFfi<A>> {
        Ok(A("Hello world".into()).into())
    }

    #[wasm_bindgen(js_name = hello_human)]
    pub async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }

    #[wasm_bindgen(js_name = Human)]
    #[derive(Clone)]
    pub struct Human {
        name: String,
    }
    #[wasm_bindgen(js_class = Human)]
    impl Human {
        #[wasm_bindgen(constructor)]
        pub fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        #[wasm_bindgen(js_name = anon)]
        pub async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        #[wasm_bindgen(js_name = hello)]
        pub fn hello(&self) -> Result<A> {
            Ok(A(format!("hello, {}", self.name)))
        }
        #[wasm_bindgen(js_name = bye)]
        pub async fn bye(&self) -> Result<String> {
            Ok(format!("bye, {}", self.name))
        }
    }
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[wasm_bindgen(js_name = A)]
pub struct A(String);
fn main() {}
