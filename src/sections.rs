/// File defining the section / subsection / subsubsection of the file
///
// use core::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

/// Temporary
pub type Core = usize; // Should be an Enum
pub type LatexFile = File;

pub struct Section {
    /// The title of the section
    pub title: String,
    /// 0 -> Section, 1 -> SubSection, 2 -> SubSubSection
    rank: usize,
    /// The content of the section
    content: Vec<Core>,
}

impl Section {
    /// Returns a new (sub, subsub)Section depending on the rank
    fn new(title: String, rank: usize) -> Self {
        assert!(rank <= 2);
        Section {
            title: title,
            rank: rank,
            content: Vec::new(),
        }
    }

    /// Returns a new Section
    pub fn new_section(title: String) -> Self {
        Section::new(title, 0)
    }

    /// Returns a new SubSection
    pub fn new_subsection(title: String) -> Self {
        Section::new(title, 1)
    }

    /// Returns a new SubSubSection
    pub fn new_subsubsection(title: String) -> Self {
        Section::new(title, 2)
    }

    /// Push some content in the section
    pub fn add_content(&mut self, new_content: Core) {
        self.content.push(new_content);
    }

    /// Changes the title of the Section
    pub fn change_title(&mut self, new_title: String) {
        self.title = new_title;
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
            _ => panic!("The rank of this section is not valid!"),
        }
    }
}

trait Writable {
    fn write_latex(&self, file: &mut LatexFile);
}


impl Writable for Section {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        write!(
            &mut writer,
            "{}{{{}}}\n",
             self.type_of_section(), self.title
        ).unwrap();
    }
}

#[cfg(test)]
mod tests_section {
    use super::*;

    #[test]
    fn creation_section() {
        // Simply creates 3 sections to see if it ends
        let _ = Section::new_section("Section".to_string());
        let _ = Section::new_subsection("SubSection".to_string());
        let _ = Section::new_subsubsection("SubSection".to_string());
    }
    
    #[test]
    fn getting_name() {
        let sec = Section::new_section("Section".to_string());
        assert_eq!(sec.title, "Section".to_string());
    }
    
    #[test]
    fn changing_name() {
        let mut sec = Section::new_section("Section".to_string());
        sec.change_title("NewSection".to_string());
        assert_eq!(sec.title, "NewSection".to_string());
    }

    #[test]
    fn ranks_type_of_section () {
        let s1 = Section::new_section("Section".to_string());
        let s2 = Section::new_subsection("SubSection".to_string());
        let s3 = Section::new_subsubsection("SubSection".to_string());
        assert_eq!(s1.type_of_section(), r"\section");
        assert_eq!(s2.type_of_section(), r"\subsection");
        assert_eq!(s3.type_of_section(), r"\subsubsection");
    }

}
