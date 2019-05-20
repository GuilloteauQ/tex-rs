/// Implements the includegraphics command
///
///
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::*;

#[derive(Clone)]
pub struct Graphic {
    filename: String,
    scale: f64,
    description: String,
}

impl Graphic {
    pub fn new(filename: String, description: String) -> Self {
        Graphic {
            filename,
            scale: 1.0,
            description,
        }
    }

    pub fn set_scale(&mut self, new_scale: f64) {
        self.scale = new_scale;
    }
}

impl Writable for Graphic {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        writeln!(&mut buf, "\\begin{{figure}}\n\t\\includegraphics[scale={}]{{{}}}\n\t\\caption{{{}}}\n\\end{{figure}}", self.scale, self.filename, self.description).unwrap();
    }
}
