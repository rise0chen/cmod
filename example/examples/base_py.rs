#![allow(unused_parens)]
#![allow(unused_variables)]

mod hello {
    use cmod::Result;

    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    #[pyo3::pyfunction]
    #[pyo3(name = "hello_world")]
    fn py_hello_world(py: pyo3::Python) -> pyo3::PyResult<String> {
        hello_world().map_err(cmod::ffi::py::map_err)
    }

    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }
    #[pyo3::pyfunction]
    #[pyo3(name = "hello_human")]
    fn py_hello_human(py: pyo3::Python, name: String) -> pyo3::PyResult<Human> {
        cmod::ffi::py::block_on(py, async move { hello_human(name).await.map_err(cmod::ffi::py::map_err) })
    }

    mod say {
        use cmod::Result;

        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
        #[pyo3::pyfunction]
        #[pyo3(name = "bye")]
        fn py_bye(py: pyo3::Python) -> pyo3::PyResult<String> {
            bye().map_err(cmod::ffi::py::map_err)
        }

        pub fn py_module_say(py: pyo3::Python<'_>, father: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
            let m = pyo3::types::PyModule::new(py, "say")?;
            m.add_function(pyo3::wrap_pyfunction!(py_bye, m)?)?;
            father.add_submodule(m)
        }
    }

    #[pyo3::pyclass]
    #[derive(Clone, Default)]
    struct Human {
        name: String,
    }
    impl Human {
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        fn hello(&self) -> Result<String> {
            Ok(format!("hello, {}", self.name))
        }
    }
    #[pyo3::pymethods]
    impl Human {
        #[staticmethod]
        #[pyo3(name = "new")]
        fn py_new(py: pyo3::Python, name: String) -> pyo3::PyResult<Human> {
            Self::new(name).map_err(cmod::ffi::py::map_err)
        }
        #[staticmethod]
        #[pyo3(name = "anon")]
        fn py_anon(py: pyo3::Python) -> pyo3::PyResult<Human> {
            cmod::ffi::py::block_on(py, async move { Self::anon().await.map_err(cmod::ffi::py::map_err) })
        }
        #[pyo3(name = "hello")]
        fn py_hello<'py>(this: pyo3::Py<Self>, py: pyo3::Python<'py>) -> pyo3::PyResult<String> {
            let this: Self = this.extract(py)?;
            this.hello().map_err(cmod::ffi::py::map_err)
        }
    }

    #[pyo3::pymodule]
    #[pyo3(name = "hello")]
    fn py_module_hello(py: pyo3::Python<'_>, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
        m.add_function(pyo3::wrap_pyfunction!(py_hello_world, m)?)?;
        m.add_function(pyo3::wrap_pyfunction!(py_hello_human, m)?)?;
        say::py_module_say(py, m)?;
        m.add_class::<Human>()?;
        Ok(())
    }
}

fn main() {}
