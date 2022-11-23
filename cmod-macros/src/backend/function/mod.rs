#[cfg(feature = "ffi_lua")]
mod lua;
#[cfg(feature = "ffi_py")]
mod py;

#[cfg(feature = "ffi_lua")]
pub use lua::impl_;
#[cfg(feature = "ffi_py")]
pub use py::cmod_function;