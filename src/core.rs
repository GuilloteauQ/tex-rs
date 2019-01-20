/// File to define the core of a LaTex file
///
use sections::*;
use equations::*;
use str_or_string::*;
use bloc::Bloc;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::*;
use tag::*;

// pub type LatexFile = File;


pub enum Core {
    Sec(Section),
    RawText(String),
    Equa(Equation),
    Bloc(Bloc),
    Tag(SingleTag),
}

impl Writable for Core {
    fn write_latex(&self, mut file: &mut LatexFile) {
        match self {
            &Core::Sec(ref section) => section.write_latex(&mut file),
            &Core::RawText(ref text) => text.write_latex(&mut file),
            &Core::Equa(ref eq) => eq.write_latex(&mut file),
            &Core::Bloc(ref bloc) => bloc.write_latex(&mut file),
            &Core::Tag(ref tag) => tag.write_latex(&mut file),
        }
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        match self {
            &Core::Sec(ref section) => section.write_to_buffer(&mut buf),
            &Core::RawText(ref text) => text.write_to_buffer(&mut buf),
            &Core::Equa(ref eq) => eq.write_to_buffer(&mut buf),
            &Core::Bloc(ref bloc) => bloc.write_to_buffer(&mut buf),
            &Core::Tag(ref tag) => tag.write_to_buffer(&mut buf),
        }
    }
}

impl Core {
    /// Returns a new section
    pub fn new_section<T: StrOrString>(title: T) -> Self {
        Core::Sec(Section::new_section(title.convert_string()))
    }

    /// Returns a new subsection
    pub fn new_subsection<T: StrOrString>(title: T) -> Self {
        Core::Sec(Section::new_subsection(title.convert_string()))
    }

    /// Returns a new subsubsection
    pub fn new_subsubsection<T: StrOrString>(title: T) -> Self {
        Core::Sec(Section::new_subsubsection(title.convert_string()))
    }

    /// Return a new text
    pub fn text<T: StrOrString>(raw_text: T) -> Self {
        Core::RawText(raw_text.convert_string())
    }

    /// Returns a new equation
    pub fn new_equation(eq: Equation) -> Self {
        Core::Equa(eq)
    }

    /// Returns a new Bloc
    pub fn new_bloc<T: StrOrString>(title: T) -> Self {
        Core::Bloc(Bloc::new_empty(title.convert_string()))
    }

    /// Returns a new \item tag
    pub fn item(content: Core) -> Self {
        Core::Tag(SingleTag::item(content))
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
        let t1 = Core::text("Quentin");
        t1.write_latex(&mut f);
    }
}

#[cfg(test)]
mod tests_core {
    use super::*;
    use latex_file::*;

    #[test]
    fn test_enumerate() {
        let mut f = new_latex_file("./tests_results/core/enumerate.tex");
        f.begin_document();
        let mut enumerate = Core::new_bloc("enumerate");
        for i in 0..5 {
            enumerate.add(Core::item(Core::text(format!("Blabla {}", i))));
        }
        enumerate.write_latex(&mut f);
        f.write_footer();
    }
}
