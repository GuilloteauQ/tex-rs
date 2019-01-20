/// File to define the trait Writable to write in a LaTex File
///

use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;


pub trait Writable {
    fn write_latex(&self, file: &mut LatexFile);
    fn write_to_buffer(&self, buf: &mut BufWriter<&mut LatexFile>);
}

impl Writable for String {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "{}", self.to_string()).unwrap();
    }
}
