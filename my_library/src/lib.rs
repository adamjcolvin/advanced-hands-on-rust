pub mod rand {
    pub use rand::*;
}

mod random;
pub use random::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
