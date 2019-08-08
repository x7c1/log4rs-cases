pub fn hello(name: &str) -> String {
    format!("hello, {}!", name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(hello("world"), "hello, world!");
    }
}
