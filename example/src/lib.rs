#[cmod::cmod]
pub mod hello {
    use crate::A;
    use cmod::Result;

    #[cmod::function]
    #[cmod::tags(ret)]
    pub fn hello_world() -> Result<A> {
        Ok(A("Hello world".into()))
    }
    #[cmod::function]
    #[cmod::tags(args(name))]
    pub async fn hello_human(name: String) -> Result<Human> {
        Human::new(name)
    }

    #[cmod::module]
    pub mod say {
        use cmod::Result;

        #[cmod::function]
        pub fn bye() -> Result<String> {
            Ok("say bye".into())
        }
    }

    #[cmod::class]
    #[derive(Clone)]
    pub struct Human {
        name: String,
    }
    #[cmod::methods]
    impl Human {
        #[staticmethod]
        #[cmod::tags(args(name))]
        pub fn new(name: String) -> Result<Human> {
            Ok(Human { name })
        }

        #[staticmethod]
        pub async fn anon() -> Result<Human> {
            Ok(Human { name: String::new() })
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub fn hello(&self) -> Result<A> {
            Ok(A(format!("hello, {}", self.name)))
        }
        #[classmethod]
        pub async fn bye(&self) -> Result<String> {
            Ok(format!("bye, {}", self.name))
        }
    }
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct A(String);
