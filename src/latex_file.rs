/// File implementing the LatexFile type

use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};
use str_or_string::*;

// pub type LatexFile = File;

pub struct LatexFile {
    /// The file where everything will be written
    file: File,
    /// The title of the document
    title: Option<String>,
    /// The name of the authors
    author: Option<String>,
    /// The names of the includes
    includes: Vec<String>,
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
    pub fn write_include<T: StrOrString>(&mut self, include: T) {
        let mut buf = BufWriter::new(&mut self.file);
        write!(&mut buf, "\\include{{{}}}\n", include.convert_string()).unwrap();
    }

    /// Change the title of the document
    pub fn title<T: StrOrString>(&mut self, new_title: T) {
        self.title = Some(new_title.convert_string());
    }

    /// Change the author of the document
    pub fn author<T: StrOrString>(&mut self, new_author: T) {
        self.author = Some(new_author.convert_string());
    }

    /// Removes the title of the document
    pub fn remove_title(&mut self) {
        self.title = None;
    }

    /// Removes the author of the document
    pub fn remove_author(&mut self) {
        self.author = None;
    }

    /// Adds a include
    pub fn add_include<T: StrOrString>(&mut self, include: T) {
        self.includes.push(include.convert_string());
    }

    /// Adds a list of includes
    pub fn add_include_list<T: StrOrString>(&mut self, includes: Vec<T>) {
        for include in includes.iter() {
            // self.add_include(*include);
            self.includes.push(include.convert_string());
        }
    }

    /// Writes the title, authors, includes, ... and begin the document
    pub fn begin_document(&mut self) {
        /* ----- INCLUDES ----- */
        for include in self.includes.iter() {
            let mut buf = BufWriter::new(&mut self.file);
            write!(&mut buf, "\\include{{{}}}\n", include.to_string()).unwrap();
        }

        /* ----- TITLE ----- */
        match self.title {
            None => {}
            Some(ref t) => {
                let mut buf = BufWriter::new(&mut self.file);
                write!(&mut buf, "\\title{{{}}}\n", t.to_string()).unwrap();
            }
        };

        /* ----- AUTHOR ----- */
        match self.author {
            None => {}
            Some(ref auth) => {
                let mut buf = BufWriter::new(&mut self.file);
                write!(&mut buf, "\\author{{{}}}\n", auth.to_string()).unwrap();
            }
        };

        self.write_in_file("\\begin{document}\n");
        if self.title != None || self.author != None {
            self.write_in_file("\\maketitle\n");
        }
    }
}
/// Returns a new LatexFile
pub fn new_latex_file<T: StrOrString>(filename: T) -> LatexFile {
    let f = File::create(filename.convert_string()).unwrap();
    let mut ltx_file = LatexFile {
        file: f,
        title: None,
        author: None,
        includes: Vec::new(),
        style: "article".to_string(),
    };
    ltx_file.write_header_article();
    ltx_file
}
