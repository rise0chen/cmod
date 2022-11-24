#![feature(type_ascription)]
#[cmod::cmod]
mod hello {
    use cmod::Result;

    #[cmod::function]
    fn hello_world() -> Result<String> {
        Ok("Hello world".into())
    }
    #[cmod::function]
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
    #[derive(Clone, Default)]
    struct Human {
        name: String,
    }
    #[cmod::methods]
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

fn main() {}
