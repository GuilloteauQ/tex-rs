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
    /// The names of the packages
    packages: Vec<String>,
    // The style of the document (article, book, ...)
    // style: String,
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

    /// Writes an package in the file
    pub fn write_package<T: AsRef<str>>(&mut self, package: T) {
        let mut buf = BufWriter::new(&mut self.file);
        writeln!(&mut buf, "\\usepackage{{{}}}", package.as_ref().to_string()).unwrap();
    }

    /// Change the title of the document
    pub fn title<T: AsRef<str>>(&mut self, new_title: T) {
        self.title = Some(new_title.as_ref().to_string());
    }

    /// Change the author of the document
    pub fn author<T: AsRef<str>>(&mut self, new_author: T) {
        self.author = Some(new_author.as_ref().to_string());
    }

    /// Removes the title of the document
    pub fn remove_title(&mut self) {
        self.title = None;
    }

    /// Removes the author of the document
    pub fn remove_author(&mut self) {
        self.author = None;
    }

    /// Adds a package
    pub fn add_package<T: AsRef<str>>(&mut self, package: T) {
        self.packages.push(package.as_ref().to_string());
    }

    /// Adds a list of packages
    pub fn add_package_list<T: AsRef<str>>(&mut self, packages: Vec<T>) {
        for package in packages.iter() {
            // self.add_include(*include);
            self.packages.push(package.as_ref().to_string());
        }
    }

    /// Writes the title, authors, packages, ... and begin the document
    pub fn begin_document(&mut self) {
        {
            let mut buf = BufWriter::new(&mut self.file);
            /* ----- INCLUDES ----- */
            for package in self.packages.iter() {
                writeln!(&mut buf, "\\usepackage{{{}}}", package.to_string()).unwrap();
            }

            /* ----- TITLE ----- */
            match self.title {
                None => {}
                Some(ref t) => {
                    writeln!(&mut buf, "\\title{{{}}}", t.to_string()).unwrap();
                }
            };

            /* ----- AUTHOR ----- */
            match self.author {
                None => {}
                Some(ref auth) => {
                    writeln!(&mut buf, "\\author{{{}}}", auth.to_string()).unwrap();
                }
            };

            writeln!(&mut buf, "\\date{{}}").unwrap();
        }

        self.write_in_file("\\begin{document}\n");
        if self.title != None || self.author != None {
            self.write_in_file("\\maketitle\n");
        }
    }
}
/// Returns a new LatexFile
pub fn new_latex_file<T: AsRef<str>>(filename: T) -> LatexFile {
    let f = File::create(filename.as_ref().to_string()).unwrap();
    let mut ltx_file = LatexFile {
        file: f,
        title: None,
        author: None,
        packages: Vec::new(),
        // style: "article".to_string(),
    };
    ltx_file.write_header_article();
    ltx_file
}
