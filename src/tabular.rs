/// Implementation of tabular
///

use core::*;
use into_tab::*;
use writable::Writable;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;

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
        write!(&mut buf, "\\begin{{tabular}}{{{}}}\n", self.align()).unwrap();
        for line in self.content.iter() {
            write!(&mut buf, " \\hline\n").unwrap();
            line[0].write_to_buffer(&mut buf);
            for elem in line.iter().skip(1) {
                write!(&mut buf, " & ").unwrap();
                elem.write_to_buffer(&mut buf);
            }
            write!(&mut buf, " \\\\\n").unwrap();
        }
        write!(&mut buf, " \\hline\n").unwrap();
        write!(&mut buf, "\\end{{tabular}}\n").unwrap();
    }
}
