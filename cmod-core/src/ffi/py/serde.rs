use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};

pub struct FromFfi<T>(T);
impl<T> FromFfi<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<'py, T: Deserialize<'py>> FromPyObject<'py> for FromFfi<T> {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        let ret = depythonize(ob)?;
        Ok(Self(ret))
    }
}

pub struct ToFfi<T: Serialize>(T);
impl<T: Serialize> From<T> for ToFfi<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
impl<T: Serialize> ToPyObject for ToFfi<T> {
    fn to_object(&self, py: Python) -> PyObject {
        pythonize(py, &self.0).unwrap_or(py.None())
    }
}
impl<T: Serialize> IntoPy<PyObject> for ToFfi<T> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        pythonize(py, &self.0).unwrap_or(py.None())
    }
}
