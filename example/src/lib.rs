#[cmod::cmod]
mod hello {
    use cmod::Result;

    #[cmod::function]
    #[cmod::tags(ret)]
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    #[cmod::function]
    #[cmod::tags(args(name))]
    async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }

    #[cmod::module]
    mod say {
        use cmod::Result;

        #[cmod::function]
        fn bye() -> Result<String> {
            Ok("say bye".into())
        }
    }

    #[cmod::class]
    #[derive(Clone)]
    struct Human {
        name: String,
    }
    #[cmod::methods]
    impl Human {
        #[staticmethod]
        #[cmod::tags(args(name))]
        fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        #[staticmethod]
        async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        #[classmethod]
        #[cmod::tags(ret)]
        fn hello(&self) -> Result<String> {
            Ok(format!("hello, {}", self.name))
        }
        #[classmethod]
        async fn bye(&self) -> Result<String> {
            Ok(format!("bye, {}", self.name))
        }
    }
}
