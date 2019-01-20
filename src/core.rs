/// File to define the core of a LaTex file
///
use sections::*;
use equations::*;
use bloc::Bloc;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;

// pub type LatexFile = File;

pub trait Writable {
    fn write_latex(&self, file: &mut LatexFile);
    fn write_to_buffer(&self, buf: &mut BufWriter<&mut LatexFile>);
}

pub enum Core {
    Sec(Section),
    RawText(String),
    Equa(Equation),
    Bloc(Bloc),
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

impl Writable for Core {
    fn write_latex(&self, mut file: &mut LatexFile) {
        match self {
            &Core::Sec(ref section) => section.write_latex(&mut file),
            &Core::RawText(ref text) => text.write_latex(&mut file),
            &Core::Equa(ref eq) => eq.write_latex(&mut file),
            &Core::Bloc(ref bloc) => bloc.write_latex(&mut file),
        }
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        match self {
            &Core::Sec(ref section) => section.write_to_buffer(&mut buf),
            &Core::RawText(ref text) => text.write_to_buffer(&mut buf),
            &Core::Equa(ref eq) => eq.write_to_buffer(&mut buf),
            &Core::Bloc(ref bloc) => bloc.write_to_buffer(&mut buf),
        }
    }
}

impl Core {
    /// Returns a new section
    pub fn new_section(title: &str) -> Self {
        Core::Sec(Section::new_section(title))
    }

    /// Returns a new subsection
    pub fn new_subsection(title: &str) -> Self {
        Core::Sec(Section::new_subsection(title))
    }

    /// Returns a new subsubsection
    pub fn new_subsubsection(title: &str) -> Self {
        Core::Sec(Section::new_subsubsection(title))
    }

    /// Return a new text
    pub fn new_raw_text(raw_text: &str) -> Self {
        Core::RawText(raw_text.to_string())
    }

    /// Returns a new equation
    pub fn new_equation(eq: Equation) -> Self {
        Core::Equa(eq)
    }

    /// Returns a new Bloc
    pub fn new_bloc(title: &str) -> Self {
        Core::Bloc(Bloc::new_empty(title))
    }

    /// Add an element to the content, if possible
    pub fn add(&mut self, element: Core) {
        match self {
            &mut Core::Sec(ref mut section) => section.add_content(element),
            &mut Core::Bloc(ref mut bloc) => bloc.add(element),
            _ => panic!("No method 'add' for this type of data")
        }
    }
}

#[cfg(test)]
mod tests_raw_text {
    use super::*;
    use latex_file::*;

    #[test]
    fn simple_write_text() {
        let mut f = new_latex_file("./tests_results/raw_texts/simple_write.tex");
        let t1 = Core::new_raw_text("Quentin");
        t1.write_latex(&mut f);
    }
}
