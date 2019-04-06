use latex_file::*;
use std::fs::File;
use std::io::prelude::*;
/// File defining a method to import content from an other file
///
use std::io::BufWriter;
use std::io::Write;
use writable::*;

// If we want to import code

#[derive(Clone)]
pub struct Code {
    filename: String,
    language: String,
}

// I'm actually not sure that we want to ba able to change
// the filename or the extension
impl Code {
    /// Returns a new Code struct
    pub fn new(filename: String, language: String) -> Self {
        Code { filename, language }
    }
}

impl Writable for Code {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(
            &mut buf,
            "\\lstinputlisting[language={}]{{{}}}\n",
            self.language, self.filename
        )
        .unwrap();
    }
}
