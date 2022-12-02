use mlua::prelude::*;
use serde::{Deserialize, Serialize};

/// 从外部语言转为Rust类型
pub struct FromFfi<T>(T);
impl<T> FromFfi<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<'lua, T: Deserialize<'lua>> FromLua<'lua> for FromFfi<T> {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        lua.from_value(lua_value).map(|x| Self(x))
    }
}

/// 从Rust类型转为外部语言
pub struct ToFfi<T: Serialize>(T);
impl<T:Serialize> From<T> for ToFfi<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
impl<'lua, T: Serialize> ToLua<'lua> for ToFfi<T> {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.to_value(&self.0)
    }
}
