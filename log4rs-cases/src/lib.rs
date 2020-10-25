mod error;
pub mod rotate;
pub use crate::error::Result;

pub fn hello(name: &str) -> String {
    format!("hello, {}!", name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::hello("world"), "hello, world!");
    }
}
