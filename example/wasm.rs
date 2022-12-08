#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub mod hello {
    use wasm_bindgen::prelude::*;
    type JResult<T> = std::result::Result<T, JsError>;
    type Result<T> = std::result::Result<T, String>;
    use crate::A;
    pub fn hello_world() -> Result<A> {
        Ok(A("Hello world".into()))
    }
    pub fn wasm_hello_world() -> JResult<cmod::ffi::wasm::ToFfi<A>> {
        hello_world().map_err(cmod::ffi::wasm::map_err).map(cmod::ffi::wasm::ToFfi::from)
    }
    #[automatically_derived]
    const __wasm_bindgen_generated_hello_world__const: () = {
        pub unsafe extern "C" fn __wasm_bindgen_generated_hello_world() -> <JResult<
            cmod::ffi::wasm::ToFfi<A>,
        > as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
            let _ret = {
                let _ret = wasm_hello_world();
                _ret
            };
            <JResult<
                cmod::ffi::wasm::ToFfi<A>,
            > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
        }
    };
    pub async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }
    pub async fn wasm_hello_human(
        name: cmod::ffi::wasm::FromFfi<String>,
    ) -> JResult<Human> {
        hello_human(name.into_inner()).await.map_err(cmod::ffi::wasm::map_err)
    }
    #[automatically_derived]
    const __wasm_bindgen_generated_hello_human__const: () = {
        pub unsafe extern "C" fn __wasm_bindgen_generated_hello_human(
            arg0: <cmod::ffi::wasm::FromFfi<
                String,
            > as wasm_bindgen::convert::FromWasmAbi>::Abi,
        ) -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <cmod::ffi::wasm::FromFfi<
                                String,
                            > as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                        };
                        let _ret = wasm_hello_human(arg0);
                        <JResult<
                            Human,
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                _ret,
            )
        }
    };
    pub struct Human {
        name: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Human {
        #[inline]
        fn clone(&self) -> Human {
            Human {
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribe for Human {
        fn describe() {
            use wasm_bindgen::__wbindgen_if_not_std;
            use wasm_bindgen::describe::*;
            inform(RUST_STRUCT);
            inform(5u32);
            inform(72u32);
            inform(117u32);
            inform(109u32);
            inform(97u32);
            inform(110u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::IntoWasmAbi for Human {
        type Abi = u32;
        fn into_abi(self) -> u32 {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::WasmRefCell;
            Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::FromWasmAbi for Human {
        type Abi = u32;
        unsafe fn from_abi(js: u32) -> Self {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
            let ptr = js as *mut WasmRefCell<Human>;
            assert_not_null(ptr);
            let js = Box::from_raw(ptr);
            (*js).borrow_mut();
            js.into_inner()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::convert::From<Human> for wasm_bindgen::JsValue {
        fn from(value: Human) -> Self {
            let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __wbg_human_new(_: u32) -> u32 {
                {
                    ::std::rt::begin_panic(
                        "cannot convert to JsValue outside of the wasm target",
                    )
                }
            }
            unsafe {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    __wbg_human_new(ptr),
                )
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefFromWasmAbi for Human {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::Ref<'static, Human>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Human>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefMutFromWasmAbi for Human {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RefMut<'static, Human>;
        unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Human>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow_mut()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionIntoWasmAbi for Human {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionFromWasmAbi for Human {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl Human {
        pub fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }
        pub async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        pub fn hello(&self) -> Result<A> {
            Ok(
                A({
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["hello, "],
                            &[::core::fmt::ArgumentV1::new_display(&self.name)],
                        ),
                    );
                    res
                }),
            )
        }
        pub async fn bye(&self) -> Result<String> {
            Ok({
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["bye, "],
                        &[::core::fmt::ArgumentV1::new_display(&self.name)],
                    ),
                );
                res
            })
        }
    }
    impl Human {
        pub fn wasm_new(name: cmod::ffi::wasm::FromFfi<String>) -> JResult<Human> {
            #[automatically_derived]
            const __wasm_bindgen_generated_Human__new__const: () = {
                pub unsafe extern "C" fn __wasm_bindgen_generated_Human__new(
                    arg0: <cmod::ffi::wasm::FromFfi<
                        String,
                    > as wasm_bindgen::convert::FromWasmAbi>::Abi,
                ) -> <JResult<Human> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = {
                        let arg0 = unsafe {
                            <cmod::ffi::wasm::FromFfi<
                                String,
                            > as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                        };
                        let _ret = Human::wasm_new(arg0);
                        _ret
                    };
                    <JResult<
                        Human,
                    > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                }
            };
            Self::new(name.into_inner()).map_err(cmod::ffi::wasm::map_err)
        }
        pub async fn wasm_anon() -> JResult<Human> {
            #[automatically_derived]
            const __wasm_bindgen_generated_Human_anon__const: () = {
                pub unsafe extern "C" fn __wasm_bindgen_generated_Human_anon() -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = wasm_bindgen_futures::future_to_promise(async move {
                            {
                                let _ret = Human::wasm_anon();
                                <JResult<
                                    Human,
                                > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                                    _ret.await,
                                )
                            }
                        })
                        .into();
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                }
            };
            Self::anon().await.map_err(cmod::ffi::wasm::map_err)
        }
        pub fn wasm_hello(&self) -> JResult<cmod::ffi::wasm::ToFfi<A>> {
            #[automatically_derived]
            const __wasm_bindgen_generated_Human_hello__const: () = {
                pub unsafe extern "C" fn __wasm_bindgen_generated_Human_hello(
                    me: u32,
                ) -> <JResult<
                    cmod::ffi::wasm::ToFfi<A>,
                > as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = {
                        let me = unsafe {
                            <Human as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                me,
                            )
                        };
                        let me = &*me;
                        let _ret = me.wasm_hello();
                        _ret
                    };
                    <JResult<
                        cmod::ffi::wasm::ToFfi<A>,
                    > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                }
            };
            self.hello()
                .map_err(cmod::ffi::wasm::map_err)
                .map(cmod::ffi::wasm::ToFfi::from)
        }
        pub async fn wasm_bye(&self) -> JResult<String> {
            #[automatically_derived]
            const __wasm_bindgen_generated_Human_bye__const: () = {
                pub unsafe extern "C" fn __wasm_bindgen_generated_Human_bye(
                    me: u32,
                ) -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                    let _ret = wasm_bindgen_futures::future_to_promise(async move {
                            {
                                let me = unsafe {
                                    <Human as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                        me,
                                    )
                                };
                                let me = &*me;
                                let _ret = me.wasm_bye();
                                <JResult<
                                    String,
                                > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                                    _ret.await,
                                )
                            }
                        })
                        .into();
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                }
            };
            self.bye().await.map_err(cmod::ffi::wasm::map_err)
        }
    }
}
use serde::{Deserialize, Serialize};
pub struct A(String);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for A {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<A>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = A;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct A",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: String = match <String as _serde::Deserialize>::deserialize(
                        __e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(A(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct A with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(A(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "A",
                __Visitor {
                    marker: _serde::__private::PhantomData::<A>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for A {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "A", &self.0)
        }
    }
};
