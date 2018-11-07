/// File defining the structure of an equation
///
use std::io::BufWriter;
use std::io::Write;
use core::*;
use latex_file::*;

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
    pub fn get_symbol(symb: &str) -> Self {
        match symb {
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
    pub fn latex_code(&self) -> &str {
        match self {
            &Symbols::Equals => " = ",
            &Symbols::LessOrEquals => " \\leq ",
            &Symbols::Less => " < ",
            &Symbols::MoreOrEquals => " \\geq ",
            &Symbols::More => " > ",
            &Symbols::Diff => " \\neq ",
        }
    }
}

fn is_op(s: &str) -> bool {
    s == "==" || s == "=" || s == "<" || s == "<=" || s == ">" || s == ">=" || s == "!="
        || s == "<>"
}

#[derive(Debug)]
pub enum EquationElements {
    Text(String),
    Symb(Symbols),
}

impl EquationElements {
    fn get_enum(s: &str) -> Self {
        if is_op(&s) {
            return EquationElements::Symb(Symbols::get_symbol(&s));
        } else {
            return EquationElements::Text(s.to_string());
        }
    }
}

pub type Equation = Vec<EquationElements>;

/// Returns an Equation from a vector of str
fn new_equation(vec: &Vec<&str>) -> Equation {
    vec.iter().map(|s| EquationElements::get_enum(s)).collect()
}

impl Writable for EquationElements {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        let st = match self {
            &EquationElements::Text(ref s) => s,
            &EquationElements::Symb(ref s) => s.latex_code(),
        };

        write!(&mut buf, "{}", st).unwrap();
    }
}

impl Writable for Equation {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "\\begin{{equation}}\n",).unwrap();
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
        assert_eq!(Symbols::get_symbol("=="), Symbols::Equals);
        assert_eq!(Symbols::get_symbol("="), Symbols::Equals);
        assert_eq!(Symbols::get_symbol(">="), Symbols::MoreOrEquals);
        assert_eq!(Symbols::get_symbol("<="), Symbols::LessOrEquals);
        assert_eq!(Symbols::get_symbol("<"), Symbols::Less);
        assert_eq!(Symbols::get_symbol(">"), Symbols::More);
        assert_eq!(Symbols::get_symbol("!="), Symbols::Diff);
        assert_eq!(Symbols::get_symbol("<>"), Symbols::Diff);
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
        let vec = vec!["a", "=", "b"];
        let eq = new_equation(&vec);
        eq.write_latex(&mut f);
    }

    #[test]
    fn multiple_equals() {
        let mut f = new_latex_file("./tests_results/equations/multiple_equals.tex");
        let eq = new_equation(&vec!["1", ">=", "0", "=", "x"]);
        eq.write_latex(&mut f);
    }

}
