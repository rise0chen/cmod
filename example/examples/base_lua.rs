#![allow(unused_parens)]
#![allow(unused_variables)]

/// 欢迎
mod hello {
    use cmod::Result;

    /// 你好 世界
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    fn lua_hello_world<'lua>(lua: &'lua mlua::Lua, (): ()) -> mlua::Result<String> {
        hello_world().map_err(cmod::ffi::lua::map_err)
    }

    /// 问好
    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }
    async fn lua_hello_human<'lua>(lua: &'lua mlua::Lua, (name): (String)) -> mlua::Result<Human> {
        hello_human(name).await.map_err(cmod::ffi::lua::map_err)
    }

    mod say {
        use cmod::Result;

        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
        fn lua_bye<'lua>(lua: &'lua mlua::Lua, (): ()) -> mlua::Result<String> {
            bye().map_err(cmod::ffi::lua::map_err)
        }

        pub fn lua_module_say<'lua>(lua: &'lua mlua::Lua, father: &mlua::Table) -> mlua::Result<()> {
            let m = lua.create_table()?;
            m.set("bye", lua.create_function(lua_bye)?)?;
            father.set("say", m)
        }
    }

    #[derive(Clone)]
    /// 人
    struct Human {
        /// 姓名
        name: String,
    }
    impl Human {
        /// 创建实例
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        /// 创建匿名者
        async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        /// 问好
        fn hello(&self) -> Result<String> {
            Ok(format!("hello, {}", self.name))
        }
    }
    impl mlua::UserData for Human {
        fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_function("new", |lua, (name): (String)| Self::new(name).map_err(cmod::ffi::lua::map_err));
            methods.add_async_function("anon", |lua, (): ()| async { Self::anon().await.map_err(cmod::ffi::lua::map_err) });
            methods.add_method("hello", |lua, this, (): ()| this.hello().map_err(cmod::ffi::lua::map_err));
        }
    }

    #[mlua::lua_module]
    fn hello(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
        let m = lua.create_table()?;
        m.set("hello_world", lua.create_function(lua_hello_world)?)?;
        m.set("hello_human", lua.create_async_function(lua_hello_human)?)?;
        say::lua_module_say(lua, &m)?;
		m.set("Human", lua.create_proxy::<Human>()?)?;
        Ok(m)
    }
}

fn main() {}
