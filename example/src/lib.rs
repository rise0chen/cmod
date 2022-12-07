#[cmod::cmod]
pub mod hello {
    use crate::A;
    use cmod::Result;

    #[cmod::function]
    #[cmod::tags(ret)]
    pub fn hello_world() -> Result<A> {
        return Ok(A("Hello world".into()));
    }

    #[cmod::class]
    #[derive(Clone)]
    pub struct Human {
        name: String,
    }
    #[cmod::methods]
    impl Human {
        #[cmod::tags(args(name))]
        pub fn new(name: String) -> Result<Human> {
            return Ok(Human { name });
        }

        pub async fn anon() -> Result<Human> {
            return Ok(Human { name: String::new() });
        }
        #[cmod::tags(ret)]
        pub fn hello(&self) -> Result<A> {
            return Ok(A(format!("hello, {}", self.name)));
        }
        pub async fn bye(&self) -> Result<String> {
            return Ok(format!("bye, {}", self.name));
        }
    }
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct A(String);
