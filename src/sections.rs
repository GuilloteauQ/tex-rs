/// File defining the section / subsection / subsubsection of the file
///
use core::*;
use latex_file::LatexFile;
use std::io::BufWriter;
use std::io::Write;
use writable::*;

// Temporary
// pub type Core = usize; // Should be an Enum
// pub type LatexFile = File;

#[derive(Clone)]
pub struct Section {
    /// The title of the section
    pub title: String,
    /// 0 -> Section, 1 -> SubSection, 2 -> SubSubSection, 3 -> paragraph
    rank: usize,
    /// The content of the section
    content: Vec<Core>,
}

impl Section {
    /// Returns a new (sub, subsub)Section depending on the rank
    fn new<T: AsRef<str>>(title: T, rank: usize) -> Self {
        assert!(rank <= 3);
        Section {
            title: title.as_ref().to_string(),
            rank: rank,
            content: Vec::new(),
        }
    }

    /// Returns a new Section
    pub fn new_section<T: AsRef<str>>(title: T) -> Self {
        Section::new(title.as_ref().to_string(), 0)
    }

    /// Returns a new SubSection
    pub fn new_subsection<T: AsRef<str>>(title: T) -> Self {
        Section::new(title.as_ref().to_string(), 1)
    }

    /// Returns a new SubSubSection
    pub fn new_subsubsection<T: AsRef<str>>(title: T) -> Self {
        Section::new(title.as_ref().to_string(), 2)
    }

    /// Returns a new Paragraph
    pub fn new_paragraph<T: AsRef<str>>(title: T) -> Self {
        Section::new(title.as_ref().to_string(), 3)
    }

    /// Push some content in the section
    pub fn add_content(&mut self, new_content: Core) {
        self.content.push(new_content);
    }

    /// Changes the title of the Section
    pub fn change_title<T: AsRef<str>>(&mut self, new_title: T) {
        self.title = new_title.as_ref().to_string();
    }

    /// Returns the title of the section
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Return the string corresponding on the rank
    fn type_of_section(&self) -> &str {
        match self.rank {
            0 => "\\section",
            1 => "\\subsection",
            2 => "\\subsubsection",
            3 => "\\paragraph",
            _ => panic!("The rank of this section is not valid!"),
        }
    }
}

impl Writable for Section {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "{}{{{}}}\n", self.type_of_section(), self.title).unwrap();
        for item in self.content.iter() {
            item.write_to_buffer(&mut buf);
        }
        write!(&mut buf, "\n").unwrap();
    }
}

#[cfg(test)]
mod tests_section {
    use super::*;
    use latex_file::*;

    #[test]
    fn creation_section() {
        // Simply creates 3 sections to see if it ends
        let _ = Section::new_section("Section");
        let _ = Section::new_subsection("SubSection");
        let _ = Section::new_subsubsection("SubSection");
    }

    #[test]
    fn getting_name() {
        let sec = Section::new_section("Section");
        assert_eq!(sec.title, "Section");
    }

    #[test]
    fn changing_name() {
        let mut sec = Section::new_section("Section");
        sec.change_title("NewSection");
        assert_eq!(sec.title, "NewSection");
    }

    #[test]
    fn ranks_type_of_section() {
        let s1 = Section::new_section("Section");
        let s2 = Section::new_subsection("SubSection");
        let s3 = Section::new_subsubsection("SubSection");
        assert_eq!(s1.type_of_section(), r"\section");
        assert_eq!(s2.type_of_section(), r"\subsection");
        assert_eq!(s3.type_of_section(), r"\subsubsection");
    }

    #[test]
    fn simple_write_in_file() {
        let mut f = new_latex_file("./tests_results/sections/section_simple_test.tex");
        f.begin_document();
        let s1 = Section::new_section("Section1");
        s1.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn nested_write_in_file() {
        let mut f = new_latex_file("./tests_results/sections/section_nested_test.tex");
        f.begin_document();
        let mut s1 = Section::new_section("Section1");
        let s2 = Section::new_subsection("subsect");
        s1.add_content(Core::Sec(s2));
        s1.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn complex_nested_write_in_file() {
        let mut f = new_latex_file("./tests_results/sections/section_complex_nested_test.tex");
        f.author("Quentin");
        f.title("Test Nested Write in File");
        f.add_include("graphics");
        f.begin_document();
        let mut s1 = Section::new_section("Section1");
        let mut s2 = Section::new_subsection("subsect");
        let s3 = Section::new_subsubsection("subsubsubsub");
        let s4 = Section::new_subsection("SUB SEC 2");
        s2.add_content(Core::Sec(s3));
        assert_eq!(s2.content.len(), 1);
        s1.add_content(Core::Sec(s2));
        s1.add_content(Core::Sec(s4));
        assert_eq!(s1.content.len(), 2);
        s1.write_latex(&mut f);
        f.write_footer();
    }

}
