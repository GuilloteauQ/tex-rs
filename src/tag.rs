use core::*;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::*;

#[derive(Clone)]
pub struct SingleTag {
    /// Name of the Tag
    name: String,
    /// The content of the tag
    content: Box<Core>,
}

impl SingleTag {
    /// Returns a new SingleTag
    fn new(name: String, content: Core) -> Self {
        SingleTag {
            name,
            content: Box::new(content),
        }
    }

    /// Returns a \item tag
    pub fn item(content: Core) -> Self {
        SingleTag::new("item".to_string(), content)
    }
}

impl Writable for SingleTag {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "\\{} ", self.name).unwrap();
        self.content.write_to_buffer(&mut buf);
        writeln!(&mut buf).unwrap();
    }
}
