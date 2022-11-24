#![feature(type_ascription)]
#[cmod::cmod]
mod hello {
    use cmod::Result;

    #[cmod::cmod_function]
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    #[cmod::cmod_function]
    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }

    #[cmod::cmod_module]
    mod say {
        use cmod::Result;

        #[cmod::cmod_function]
        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
    }

    #[cmod::cmod_class]
    #[derive(Clone, Default)]
    struct Human {
        name: String,
    }
    #[cmod::cmod_methods]
    impl Human {
        #[staticmethod]
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        #[staticmethod]
        async fn anon() -> Result<Human> {
            Ok(Human {
                name: String::new(),
            })
        }
        #[classmethod]
        fn hello(&self) -> Result<String> {
            Ok(format!("hello, {}", self.name))
        }
    }
}
