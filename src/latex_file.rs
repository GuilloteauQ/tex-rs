/// File implementing the LatexFile type

use std::fs::File;

pub type LatexFile = File;

pub fn new_latex_file(filename: &str) -> LatexFile {
    File::create(filename).unwrap()
}
