pub mod sections;
pub mod equations;
pub mod core;
pub mod latex_file;
pub mod operators;
pub mod displays;
pub mod bloc;
pub mod writable;
pub mod symbols;
pub mod tag;
pub mod tabular;
pub mod into_tab;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
