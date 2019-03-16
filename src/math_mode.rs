use latex_file::LatexFile;
/// Math mode
use std::io::BufWriter;
use std::io::Write;
use writable::*;

#[derive(Clone)]
pub struct MathContent {
    content: String,
}

impl MathContent {
    pub fn new(content: String) -> Self {
        MathContent { content }
    }
}

impl Writable for MathContent {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "${}$", self.content).unwrap();
    }
}
