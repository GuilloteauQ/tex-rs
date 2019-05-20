/// Implementation of tabular
///
use core::*;
use into_tab::*;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::Writable;

#[derive(Clone)]
pub struct Tabular {
    /// Content
    content: Vec<Vec<Core>>,
}

impl Tabular {
    pub fn new<T: IntoTab>(content: &T) -> Self {
        Tabular {
            content: content.into_tab(),
        }
    }

    fn align(&self) -> String {
        let size = self.content[0].len();
        let mut cols = String::from("|");
        for _ in 0..size {
            cols.push_str(" c |");
        }
        cols
    }
}

impl Writable for Tabular {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        writeln!(&mut buf, "\\begin{{tabular}}{{{}}}", self.align()).unwrap();
        for line in self.content.iter() {
            writeln!(&mut buf, " \\hline").unwrap();
            line[0].write_to_buffer(&mut buf);
            for elem in line.iter().skip(1) {
                write!(&mut buf, " & ").unwrap();
                elem.write_to_buffer(&mut buf);
            }
            writeln!(&mut buf, " \\\\").unwrap();
        }
        writeln!(&mut buf, " \\hline").unwrap();
        writeln!(&mut buf, "\\end{{tabular}}").unwrap();
    }
}
