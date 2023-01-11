mod serde;

pub use self::serde::{FromFfi, ToFfi};
use core::future::Future;
use pyo3::exceptions::PyTypeError;
pub use pyo3::prelude::*;

pub fn map_err(msg: String) -> PyErr {
    PyTypeError::new_err(msg)
}

pub fn block_on<F, T>(py: Python, f: F) -> PyResult<T>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: Send + Sync + 'static,
{
    let _ = py;
    futures_executor::block_on(f)
}
