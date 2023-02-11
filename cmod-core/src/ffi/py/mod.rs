mod serde;

pub use self::serde::{FromFfi, ToFfi};
use core::future::Future;
use pyo3::exceptions::PyTypeError;
pub use pyo3::prelude::*;
use tokio::runtime::Runtime;

pub fn map_err(msg: String) -> PyErr {
    PyTypeError::new_err(msg)
}

pub fn init_runtime(runtime: &'static Runtime) -> Result<(), ()> {
    pyo3_asyncio::tokio::init_with_runtime(runtime)
}

#[cfg(feature = "ffi_py_asyncio")]
pub fn block_on<F, T>(py: Python, f: F) -> PyResult<&PyAny>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: IntoPy<PyObject> + Send + Sync + 'static,
{
    pyo3_asyncio::tokio::future_into_py(py, f)
}
#[cfg(not(feature = "ffi_py_asyncio"))]
pub fn block_on<F, T>(py: Python, f: F) -> PyResult<T>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: IntoPy<PyObject> + Send + Sync + 'static,
{
    pyo3_asyncio::tokio::run(py, f)
}
