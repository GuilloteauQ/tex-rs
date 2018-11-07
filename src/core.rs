/// File to define the core of a LaTex file
///
use sections::*;
use equations::*;
use latex_file::*;
use std::fs::File;
use std::io::BufWriter;

// pub type LatexFile = File;

pub trait Writable {
    fn write_latex(&self, file: &mut LatexFile);
    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>);
}

pub enum Core {
    Sec(Section),
    // RawText(String),
    Equa(Equation),
}

impl Writable for Core {
    fn write_latex(&self, mut file: &mut LatexFile) {
        match self {
            &Core::Sec(ref section) => section.write_latex(&mut file),
            // &Core::RawText(text) => text.write_latex(&mut file),
            &Core::Equa(ref eq) => eq.write_latex(&mut file),
        }
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        match self {
            &Core::Sec(ref section) => section.write_to_buffer(&mut buf),
            // &Core::RawText(text) => text.write_latex(&mut file),
            &Core::Equa(ref eq) => eq.write_to_buffer(&mut buf),
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
    /*
    pub fn new_raw_text(raw_text: String) -> Self {
        Core::RawText(raw_text)
    }
    */

    /// Returns a new equation
    pub fn new_equation(eq: Equation) -> Self {
        Core::Equa(eq)
    }
}
