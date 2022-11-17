#[cmod::cmod]
/// 欢迎
mod hello {
    use cmod::Result;

    #[cmod_function]
    /// 你好 世界
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    #[cmod_function]
    /// 问好
    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }

    #[cmod_module]
    mod say {
        use cmod::Result;

        #[cmod_function]
        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
    }

    #[cmod_class]
    #[derive(Clone, Default)]
    /// 人
    struct Human {
        /// 姓名
        name: String,
    }
    #[cmod_methods]
    impl Human {
        #[staticmethod]
        /// 创建实例
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        #[staticmethod]
        /// 创建匿名者
        async fn anon() -> Result<Human> {
            Ok(Human {
                name: String::new(),
            })
        }
        #[classmethod]
        /// 问好
        fn hello(&self) -> Result<String> {
            Ok(format!("hello, {}", self.name))
        }
    }
}

fn main() {}
