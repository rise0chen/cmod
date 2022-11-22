mod serde;

pub use self::serde::{FromFfi, ToFfi};
pub use mlua::prelude::*;

pub fn map_err(msg: String) -> LuaError {
    LuaError::RuntimeError(msg)
}
