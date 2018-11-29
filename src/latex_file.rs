/// File implementing the LatexFile type

use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};

// pub type LatexFile = File;

pub struct LatexFile {
    /// The file where everything will be written
    file: File,
    /// The title of the document
    title: Option<String>,
    /// The name of the authors
    author: Option<String>,
    /// The names of the includes
    includes: Option<Vec<String>>,
    /// The style of the document (article, book, ...)
    style: String,
}

impl Write for LatexFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}



impl LatexFile {
    /// Writes raw text in the file
    fn write_in_file(&mut self, line: &str) {
        let mut buf = BufWriter::new(&mut self.file);
        write!(&mut buf, "{}", line.to_string()).unwrap();
    }

    /// Writes the document class if ir is an article
    pub fn write_header_article(&mut self) {
        self.write_in_file("\\documentclass[a4paper,11pt]{article}\n");
    }

    /// Writes the footer of the class
    pub fn write_footer(&mut self) {
        self.write_in_file("\\end{document}\n");
    }

    /// Writes an include in the file
    fn write_include(&mut self, include: &str) {
        let mut buf = BufWriter::new(&mut self.file);
        write!(&mut buf, "\\include{{{}}}\n", include.to_string()).unwrap();
    }

    /// Change the title of the document
    pub fn change_title(&mut self, new_title: &str) {
        self.title = Some(new_title.to_string());
    }

    /// Change the author of the document
    pub fn change_author(&mut self, new_author: &str) {
        self.author = Some(new_author.to_string());
    }

    /// Removes the title of the document
    pub fn remove_title(&mut self) {
        self.title = None;
    }

    /// Removes the author of the document
    pub fn remove_author(&mut self) {
        self.author = None;
    }


    /// Writes the title, authors, includes, ... and begin the document
    pub fn begin_document(&mut self) {
        /* ----- INCLUDES ----- */
        match self.includes {
            None => {},
            Some(ref vec) => {
                for include in vec.iter(){
                    let mut buf = BufWriter::new(&mut self.file);
                    write!(&mut buf, "\\include{{{}}}\n", include.to_string()).unwrap();
                }
            }
        };

        /* ----- TITLE ----- */
        match self.title {
            None => {},
            Some(ref t) => {
                let mut buf = BufWriter::new(&mut self.file);
                write!(&mut buf, "\\title{{{}}}\n", t.to_string()).unwrap();
            }
        };

        /* ----- AUTHOR ----- */
        match self.author {
            None => {},
            Some(ref auth) => {
                let mut buf = BufWriter::new(&mut self.file);
                write!(&mut buf, "\\author{{{}}}\n", auth.to_string()).unwrap();
            }
        };


    self.write_in_file("\\begin{document}\n");
    self.write_in_file("\\maketitle\n");
    }
}
/// Returns a new LatexFile
pub fn new_latex_file(filename: &str) -> LatexFile {

    let f = File::create(filename).unwrap();
    let mut ltx_file = LatexFile {
        file: f,
        title: None,
        author: None,
        includes: None,
        style:  "article".to_string(),
    };
    ltx_file.write_header_article();
    ltx_file
}


