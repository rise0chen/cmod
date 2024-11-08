use mlua::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

/// 从外部语言转为Rust类型
pub struct FromFfi<T>(T);
impl<T> FromFfi<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T: DeserializeOwned> FromLua for FromFfi<T> {
    fn from_lua(lua_value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let options = LuaDeserializeOptions::new().deny_unsupported_types(false);
        lua.from_value_with(lua_value, options).map(|x| Self(x))
    }
}

/// 从Rust类型转为外部语言
pub struct ToFfi<T>(T);
impl<T> From<T> for ToFfi<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
impl<T: Serialize> IntoLua for ToFfi<T> {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let options = LuaSerializeOptions::new().serialize_none_to_null(false).serialize_unit_to_null(false);
        lua.to_value_with(&self.0, options)
    }
}
