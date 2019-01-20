pub mod sections;
pub mod equations;
pub mod core;
pub mod latex_file;
pub mod operators;
pub mod displays;
pub mod str_or_string;
pub mod bloc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
