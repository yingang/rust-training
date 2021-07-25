pub mod user;
pub mod strtok;
pub mod ticket;
pub mod fibonacci;
pub mod encoder;
pub mod actor;  // 如果这里不用pub，会产生更多定义未使用方面的warning

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
