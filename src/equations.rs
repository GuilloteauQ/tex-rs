/// File defining the structure of an equation
///
use std::io::BufWriter;
use std::io::Write;
use core::*;
use latex_file::*;
use operators::*;
use displays::*;
use std::fmt;
use str_or_string::*;

#[derive(Debug, PartialEq)]
pub enum Symbols {
    /// =
    Equals,
    /// <=
    LessOrEquals,
    /// <
    Less,
    /// >=
    MoreOrEquals,
    /// >
    More,
    /// !=
    Diff,
}

impl Symbols {
    /// Returns the enum corresponding to the String
    pub fn get_symbol(symb: String) -> Self {
        match symb.as_ref() {
            "=" => Symbols::Equals,
            "==" => Symbols::Equals,
            "<=" => Symbols::LessOrEquals,
            "<" => Symbols::Less,
            ">=" => Symbols::MoreOrEquals,
            ">" => Symbols::More,
            "!=" => Symbols::Diff,
            "<>" => Symbols::Diff,
            _ => panic!("get_symbol: The symbol {} is not a valid symbol !", symb),
        }
    }

    /// Returns the string corresponding to the enum
    pub fn get_string(&self) -> &str {
        match self {
            &Symbols::Equals => "=",
            &Symbols::LessOrEquals => "<=",
            &Symbols::Less => "<",
            &Symbols::MoreOrEquals => ">=",
            &Symbols::More => ">",
            &Symbols::Diff => "!=",
        }
    }

    /// Returns the LaTeX "code" for each item
    pub fn latex_code(&self) -> String {
        let x = match self {
            &Symbols::Equals => " = ",
            &Symbols::LessOrEquals => " \\leq ",
            &Symbols::Less => " < ",
            &Symbols::MoreOrEquals => " \\geq ",
            &Symbols::More => " > ",
            &Symbols::Diff => " \\neq ",
        };
        String::from(x)
    }
}

fn is_op(s: &String) -> bool {
    s == "==" || s == "=" || s == "<" || s == "<=" || s == ">" || s == ">=" || s == "!="
        || s == "<>"
}

pub enum EquationElements {
    Text(String),
    Symb(Symbols),
    Operator(Operators),
}

impl EquationElements {
    fn get_enum(elem: String) -> Self {
        if is_op(&elem) {
            return EquationElements::Symb(Symbols::get_symbol(elem));
        } else {
            return EquationElements::Text(elem);
        }
    }
}

pub type Equation = Vec<EquationElements>;

/// Returns an Equation from a vector of str
pub fn new_equation<T: StrOrString>(vec: &Vec<T>) -> Equation {
    vec.iter()
        .map(|s| EquationElements::get_enum(s.convert_string()))
        .collect()
}

impl Writable for EquationElements {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        match self {
            &EquationElements::Text(ref s) => write!(&mut buf, "{} ", s).unwrap(),
            &EquationElements::Symb(ref s) => write!(&mut buf, "{} ", s.latex_code()).unwrap(),
            &EquationElements::Operator(ref s) => write!(&mut buf, "{} ", s.latex_code()).unwrap(),
        }
    }
}

impl Writable for Equation {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "\\begin{{equation}}\n\\displaystyle ",).unwrap();
        for item in self.iter() {
            item.write_to_buffer(&mut buf);
        }
        write!(&mut buf, "\n\\end{{equation}}\n",).unwrap();
    }
}

#[cfg(test)]
mod tests_symbols {
    use super::*;

    #[test]
    fn getting_symbols() {
        assert_eq!(Symbols::get_symbol("==".to_string()), Symbols::Equals);
        assert_eq!(Symbols::get_symbol("=".to_string()), Symbols::Equals);
        assert_eq!(Symbols::get_symbol(">=".to_string()), Symbols::MoreOrEquals);
        assert_eq!(Symbols::get_symbol("<=".to_string()), Symbols::LessOrEquals);
        assert_eq!(Symbols::get_symbol("<".to_string()), Symbols::Less);
        assert_eq!(Symbols::get_symbol(">".to_string()), Symbols::More);
        assert_eq!(Symbols::get_symbol("!=".to_string()), Symbols::Diff);
        assert_eq!(Symbols::get_symbol("<>".to_string()), Symbols::Diff);
    }

    #[test]
    fn getting_strings() {
        assert_eq!(Symbols::get_string(&Symbols::Equals), "=");
        assert_eq!(Symbols::get_string(&Symbols::More), ">");
        assert_eq!(Symbols::get_string(&Symbols::MoreOrEquals), ">=");
        assert_eq!(Symbols::get_string(&Symbols::LessOrEquals), "<=");;
        assert_eq!(Symbols::get_string(&Symbols::Less), "<");
        assert_eq!(Symbols::get_string(&Symbols::Diff), "!=");
    }

}

#[cfg(test)]
mod tests_equations {
    use super::*;

    #[test]
    fn strings_to_equation() {
        let vec = vec!["a", "!=", "b"];
        let eq = new_equation(&vec);
        assert_eq!(eq.len(), 3);
    }

    #[test]
    fn simple_write() {
        let mut f = new_latex_file("./tests_results/equations/simple_eq.tex");
        f.begin_document();
        let vec = vec!["a", "=", "b"];
        let eq = new_equation(&vec);
        eq.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn multiple_equals() {
        let mut f = new_latex_file("./tests_results/equations/multiple_equals.tex");
        f.begin_document();
        let eq = new_equation(&vec!["1", ">=", "0", "=", "x"]);
        eq.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn multiple_frac() {
        let mut f = new_latex_file("./tests_results/equations/frac.tex");
        f.begin_document();
        let v = vec![frac("4", "2"), "=".to_string(), "2".to_string()];
        let eq = new_equation(&v);
        eq.write_latex(&mut f);
        f.write_footer();
    }

}
