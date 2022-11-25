#![feature(prelude_import)]
#![feature(type_ascription)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod hello {
    use cmod::Result;
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    fn py_hello_world(py: pyo3::Python) -> pyo3::PyResult<String> {
        hello_world().map_err(cmod::ffi::py::map_err).map(|x| x.into())
    }
    #[doc(hidden)]
    mod py_hello_world {
        pub(crate) struct MakeDef;
        pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
    }
    const _: () = {
        use ::pyo3 as _pyo3;
        impl py_hello_world::MakeDef {
            const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                "hello_world\0",
                _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                    __pyfunction_py_hello_world,
                ),
                "\u{0}",
            );
        }
        unsafe extern "C" fn __pyfunction_py_hello_world(
            _slf: *mut _pyo3::ffi::PyObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> *mut _pyo3::ffi::PyObject {
            let gil = _pyo3::GILPool::new();
            let _py = gil.python();
            _pyo3::callback::panic_result_into_callback_output(
                _py,
                ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                    const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                        cls_name: ::std::option::Option::None,
                        func_name: "hello_world",
                        positional_parameter_names: &[],
                        positional_only_parameters: 0usize,
                        required_positional_parameters: 0usize,
                        keyword_only_parameters: &[],
                    };
                    let mut output = [::std::option::Option::None; 0usize];
                    let (_args, _kwargs) = DESCRIPTION
                        .extract_arguments_tuple_dict::<
                            _pyo3::impl_::extract_argument::NoVarargs,
                            _pyo3::impl_::extract_argument::NoVarkeywords,
                        >(_py, _args, _kwargs, &mut output)?;
                    let mut ret = py_hello_world(_py);
                    if false {
                        use _pyo3::impl_::ghost::IntoPyResult;
                        ret.assert_into_py_result();
                    }
                    _pyo3::callback::convert(_py, ret)
                }),
            )
        }
    };
    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }
    fn py_hello_human(
        py: pyo3::Python,
        name: cmod::ffi::py::FromFfi<String>,
    ) -> pyo3::PyResult<cmod::ffi::py::ToFfi<Human>> {
        cmod::ffi::py::block_on(
            py,
            async move {
                hello_human(name.into_inner())
                    .await
                    .map_err(cmod::ffi::py::map_err)
                    .map(|x| x.into())
            },
        )
    }
    #[doc(hidden)]
    mod py_hello_human {
        pub(crate) struct MakeDef;
        pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
    }
    const _: () = {
        use ::pyo3 as _pyo3;
        impl py_hello_human::MakeDef {
            const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                "hello_human\0",
                _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                    __pyfunction_py_hello_human,
                ),
                "\u{0}",
            );
        }
        unsafe extern "C" fn __pyfunction_py_hello_human(
            _slf: *mut _pyo3::ffi::PyObject,
            _args: *mut _pyo3::ffi::PyObject,
            _kwargs: *mut _pyo3::ffi::PyObject,
        ) -> *mut _pyo3::ffi::PyObject {
            let gil = _pyo3::GILPool::new();
            let _py = gil.python();
            _pyo3::callback::panic_result_into_callback_output(
                _py,
                ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                    const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                        cls_name: ::std::option::Option::None,
                        func_name: "hello_human",
                        positional_parameter_names: &["name"],
                        positional_only_parameters: 0usize,
                        required_positional_parameters: 1usize,
                        keyword_only_parameters: &[],
                    };
                    let mut output = [::std::option::Option::None; 1usize];
                    let (_args, _kwargs) = DESCRIPTION
                        .extract_arguments_tuple_dict::<
                            _pyo3::impl_::extract_argument::NoVarargs,
                            _pyo3::impl_::extract_argument::NoVarkeywords,
                        >(_py, _args, _kwargs, &mut output)?;
                    let mut ret = py_hello_human(
                        _py,
                        _pyo3::impl_::extract_argument::extract_argument(
                            _pyo3::impl_::extract_argument::unwrap_required_argument(
                                output[0usize],
                            ),
                            &mut {
                                _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT
                            },
                            "name",
                        )?,
                    );
                    if false {
                        use _pyo3::impl_::ghost::IntoPyResult;
                        ret.assert_into_py_result();
                    }
                    _pyo3::callback::convert(_py, ret)
                }),
            )
        }
    };
    mod say {
        use cmod::Result;
        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
        fn py_bye(py: pyo3::Python) -> pyo3::PyResult<String> {
            bye().map_err(cmod::ffi::py::map_err).map(|x| x.into())
        }
        #[doc(hidden)]
        mod py_bye {
            pub(crate) struct MakeDef;
            pub const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = MakeDef::DEF;
        }
        const _: () = {
            use ::pyo3 as _pyo3;
            impl py_bye::MakeDef {
                const DEF: ::pyo3::impl_::pyfunction::PyMethodDef = _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                    "bye\0",
                    _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                        __pyfunction_py_bye,
                    ),
                    "\u{0}",
                );
            }
            unsafe extern "C" fn __pyfunction_py_bye(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::None,
                            func_name: "bye",
                            positional_parameter_names: &[],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 0usize,
                            keyword_only_parameters: &[],
                        };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args, _kwargs) = DESCRIPTION
                            .extract_arguments_tuple_dict::<
                                _pyo3::impl_::extract_argument::NoVarargs,
                                _pyo3::impl_::extract_argument::NoVarkeywords,
                            >(_py, _args, _kwargs, &mut output)?;
                        let mut ret = py_bye(_py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
        };
        pub fn py_module_say(
            py: pyo3::Python<'_>,
            father: &pyo3::types::PyModule,
        ) -> pyo3::PyResult<()> {
            let m = pyo3::types::PyModule::new(py, "say")?;
            m.add_function(
                {
                    use py_bye as wrapped_pyfunction;
                    ::pyo3::impl_::pyfunction::wrap_pyfunction(
                        &wrapped_pyfunction::DEF,
                        m,
                    )
                }?,
            )?;
            return father.add_submodule(m);
        }
    }
    struct Human {
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
    impl ::core::default::Default for Human {
        #[inline]
        fn default() -> Human {
            Human {
                name: ::core::default::Default::default(),
            }
        }
    }
    const _: () = {
        use ::pyo3 as _pyo3;
        unsafe impl _pyo3::type_object::PyTypeInfo for Human {
            type AsRefTarget = _pyo3::PyCell<Self>;
            const NAME: &'static str = "Human";
            const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
            #[inline]
            fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
                use _pyo3::type_object::LazyStaticType;
                static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
                TYPE_OBJECT.get_or_init::<Self>(py)
            }
        }
        impl _pyo3::PyClass for Human {
            type Frozen = _pyo3::pyclass::boolean_struct::False;
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a Human {
            type Holder = ::std::option::Option<_pyo3::PyRef<'py, Human>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref(obj, holder)
            }
        }
        impl<'a, 'py> _pyo3::impl_::extract_argument::PyFunctionArgument<'a, 'py>
        for &'a mut Human {
            type Holder = ::std::option::Option<_pyo3::PyRefMut<'py, Human>>;
            #[inline]
            fn extract(
                obj: &'py _pyo3::PyAny,
                holder: &'a mut Self::Holder,
            ) -> _pyo3::PyResult<Self> {
                _pyo3::impl_::extract_argument::extract_pyclass_ref_mut(obj, holder)
            }
        }
        impl _pyo3::IntoPy<_pyo3::PyObject> for Human {
            fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
                _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
            }
        }
        impl _pyo3::impl_::pyclass::PyClassImpl for Human {
            const DOC: &'static str = "\u{0}";
            const IS_BASETYPE: bool = false;
            const IS_SUBCLASS: bool = false;
            const IS_MAPPING: bool = false;
            const IS_SEQUENCE: bool = false;
            type Layout = _pyo3::PyCell<Self>;
            type BaseType = _pyo3::PyAny;
            type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<Human>;
            type PyClassMutability = <<_pyo3::PyAny as _pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as _pyo3::impl_::pycell::PyClassMutability>::MutableChild;
            type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
            type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
            type BaseNativeType = _pyo3::PyAny;
            fn items_iter() -> _pyo3::impl_::pyclass::PyClassItemsIter {
                use _pyo3::impl_::pyclass::*;
                let collector = PyClassImplCollector::<Self>::new();
                static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                    methods: &[],
                    slots: &[],
                };
                PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl Human {}
    };
    impl Human {
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }
        async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        fn hello(&self) -> Result<String> {
            Ok({
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["hello, "],
                        &[::core::fmt::ArgumentV1::new_display(&self.name)],
                    ),
                );
                res
            })
        }
    }
    impl Human {
        fn py_new(py: pyo3::Python, name: String) -> pyo3::PyResult<Human> {
            Self::new(name).map_err(cmod::ffi::py::map_err).map(|x| x.into())
        }
        fn py_anon(py: pyo3::Python) -> pyo3::PyResult<Human> {
            cmod::ffi::py::block_on(
                py,
                async move {
                    Self::anon().await.map_err(cmod::ffi::py::map_err).map(|x| x.into())
                },
            )
        }
        fn py_hello<'py>(
            this: pyo3::Py<Self>,
            py: pyo3::Python<'py>,
        ) -> pyo3::PyResult<String> {
            let this: Self = this.extract(py)?;
            this.hello().map_err(cmod::ffi::py::map_err).map(|x| x.into())
        }
    }
    const _: () = {
        use ::pyo3 as _pyo3;
        impl _pyo3::impl_::pyclass::PyMethods<Human>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Human> {
            fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
                static ITEMS: _pyo3::impl_::pyclass::PyClassItems = _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[
                        _pyo3::class::PyMethodDefType::Static(
                            _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                                    "new\0",
                                    _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                                        Human::__pymethod_new__,
                                    ),
                                    "\u{0}",
                                )
                                .flags(_pyo3::ffi::METH_STATIC),
                        ),
                        _pyo3::class::PyMethodDefType::Static(
                            _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                                    "anon\0",
                                    _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                                        Human::__pymethod_anon__,
                                    ),
                                    "\u{0}",
                                )
                                .flags(_pyo3::ffi::METH_STATIC),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::cfunction_with_keywords(
                                "hello\0",
                                _pyo3::impl_::pymethods::PyCFunctionWithKeywords(
                                    Human::__pymethod_hello__,
                                ),
                                "\u{0}",
                            ),
                        ),
                    ],
                    slots: &[],
                };
                &ITEMS
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl Human {
            unsafe extern "C" fn __pymethod_new__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::Some(
                                <Human as _pyo3::type_object::PyTypeInfo>::NAME,
                            ),
                            func_name: "new",
                            positional_parameter_names: &["name"],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 1usize,
                            keyword_only_parameters: &[],
                        };
                        let mut output = [::std::option::Option::None; 1usize];
                        let (_args, _kwargs) = DESCRIPTION
                            .extract_arguments_tuple_dict::<
                                _pyo3::impl_::extract_argument::NoVarargs,
                                _pyo3::impl_::extract_argument::NoVarkeywords,
                            >(_py, _args, _kwargs, &mut output)?;
                        let mut ret = Human::py_new(
                            _py,
                            _pyo3::impl_::extract_argument::extract_argument(
                                _pyo3::impl_::extract_argument::unwrap_required_argument(
                                    output[0usize],
                                ),
                                &mut {
                                    _pyo3::impl_::extract_argument::FunctionArgumentHolder::INIT
                                },
                                "name",
                            )?,
                        );
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_anon__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::Some(
                                <Human as _pyo3::type_object::PyTypeInfo>::NAME,
                            ),
                            func_name: "anon",
                            positional_parameter_names: &[],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 0usize,
                            keyword_only_parameters: &[],
                        };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args, _kwargs) = DESCRIPTION
                            .extract_arguments_tuple_dict::<
                                _pyo3::impl_::extract_argument::NoVarargs,
                                _pyo3::impl_::extract_argument::NoVarkeywords,
                            >(_py, _args, _kwargs, &mut output)?;
                        let mut ret = Human::py_anon(_py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
            unsafe extern "C" fn __pymethod_hello__(
                _slf: *mut _pyo3::ffi::PyObject,
                _args: *mut _pyo3::ffi::PyObject,
                _kwargs: *mut _pyo3::ffi::PyObject,
            ) -> *mut _pyo3::ffi::PyObject {
                let gil = _pyo3::GILPool::new();
                let _py = gil.python();
                _pyo3::callback::panic_result_into_callback_output(
                    _py,
                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                        let _cell = _py
                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                            .downcast::<_pyo3::PyCell<Human>>()?;
                        #[allow(clippy::useless_conversion)]
                        let _slf = ::std::convert::TryFrom::try_from(_cell)?;
                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                            cls_name: ::std::option::Option::Some(
                                <Human as _pyo3::type_object::PyTypeInfo>::NAME,
                            ),
                            func_name: "hello",
                            positional_parameter_names: &[],
                            positional_only_parameters: 0usize,
                            required_positional_parameters: 0usize,
                            keyword_only_parameters: &[],
                        };
                        let mut output = [::std::option::Option::None; 0usize];
                        let (_args, _kwargs) = DESCRIPTION
                            .extract_arguments_tuple_dict::<
                                _pyo3::impl_::extract_argument::NoVarargs,
                                _pyo3::impl_::extract_argument::NoVarkeywords,
                            >(_py, _args, _kwargs, &mut output)?;
                        let mut ret = Human::py_hello(_slf, _py);
                        if false {
                            use _pyo3::impl_::ghost::IntoPyResult;
                            ret.assert_into_py_result();
                        }
                        _pyo3::callback::convert(_py, ret)
                    }),
                )
            }
        }
    };
    fn py_module_hello(
        py: pyo3::Python<'_>,
        m: &pyo3::types::PyModule,
    ) -> pyo3::PyResult<()> {
        m.add_function(
            {
                use py_hello_world as wrapped_pyfunction;
                ::pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
            }?,
        )?;
        m.add_function(
            {
                use py_hello_human as wrapped_pyfunction;
                ::pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
            }?,
        )?;
        say::py_module_say(py, m)?;
        m.add_class::<Human>()?;
        return Ok(());
    }
    #[doc(hidden)]
    mod py_module_hello {
        pub(crate) struct MakeDef;
        pub static DEF: ::pyo3::impl_::pymodule::ModuleDef = MakeDef::make_def();
        pub const NAME: &'static str = "hello\u{0}";
        /// This autogenerated function is called by the python interpreter when importing
        /// the module.
        #[export_name = "PyInit_hello"]
        pub unsafe extern "C" fn init() -> *mut ::pyo3::ffi::PyObject {
            DEF.module_init()
        }
    }
    const _: () = {
        use ::pyo3::impl_::pymodule as impl_;
        impl py_module_hello::MakeDef {
            const fn make_def() -> impl_::ModuleDef {
                const INITIALIZER: impl_::ModuleInitializer = impl_::ModuleInitializer(
                    py_module_hello,
                );
                unsafe {
                    impl_::ModuleDef::new(py_module_hello::NAME, "\u{0}", INITIALIZER)
                }
            }
        }
    };
}
