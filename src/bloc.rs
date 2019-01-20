/// Defines a bloc in LaTex
/// for example \begin{center}...\end{center}
///

use core::*;
use str_or_string::*;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::*;


#[derive(Clone)]
pub struct Bloc {
    /// The type of the Bloc
    bloc_type : String,
    /// The content in the Bloc
    content : Vec<Core>,
}

impl Bloc {
    /// Returns a new Bloc
    pub fn new<T: StrOrString>(bloc_type: T, content: Vec<Core>) -> Self {
        Bloc {
            bloc_type: bloc_type.convert_string(),
            content: content
        }
    }

    /// Returns a new Bloc with an empty body
    pub fn new_empty<T: StrOrString>(bloc_type: T) -> Self {
        Bloc {
            bloc_type: bloc_type.convert_string(),
            content: Vec::new()
        }
    }

    /// Adds an element to the content of the bloc
    pub fn add(&mut self, element: Core) {
        self.content.push(element);
    }
}

impl Writable for Bloc {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "\\begin{{{}}}\n", self.bloc_type).unwrap();
        for item in self.content.iter() {
            item.write_to_buffer(&mut buf);
        }
        write!(&mut buf, "\n\\end{{{}}}\n", self.bloc_type).unwrap();
    }
}

#[cfg(test)]
mod tests_bloc {
    use super::*;
    use latex_file::*;

    #[test]
    fn creation_bloc_new_empty() {
        let _ = Bloc::new_empty("center");
    }

    #[test]
    fn creation_bloc_new() {
        let _ = Bloc::new("center", vec![Core::new_bloc("verbatim")]);
    }

    #[test]
    fn test_add() {
        let mut b = Bloc::new_empty("center");
        assert_eq!(b.content.len(), 0);
        b.add(Core::new_bloc("verbatim"));
        assert_eq!(b.content.len(), 1);
    }

    #[test]
    fn test_add_core() {
        let mut b = Core::new_bloc("center");
        b.add(Core::new_bloc("verbatim"));
    }

    #[test]
    fn test_write_simple_bloc_empty() {
        let mut f = new_latex_file("./tests_results/bloc/bloc_simple_test_empty.tex");
        f.begin_document();
        let b = Bloc::new_empty("center");
        b.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_write_simple_bloc_non_empty() {
        let mut f = new_latex_file("./tests_results/bloc/bloc_simple_test_non_empty.tex");
        f.begin_document();
        let mut b = Bloc::new_empty("center");
        b.add(Core::text("This is supposed to be centered"));
        b.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_write_nested_bloc_non_empty() {
        let mut f = new_latex_file("./tests_results/bloc/bloc_nested_test_non_empty.tex");
        f.begin_document();
        let mut b = Bloc::new_empty("center");
        let mut b2 = Bloc::new_empty("verbatim");
        b2.add(Core::text("To be or not to be"));
        b.add(Core::Bloc(b2));
        b.write_latex(&mut f);
        f.write_footer();
    }

}
