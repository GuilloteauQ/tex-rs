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
}

#[derive(Debug)]
pub enum EquationElements {
    Text(String),
    Symb(Symbols),
}

pub type Equation = Vec<EquationElements>;

impl Writable for EquationElements {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        let st = match self {
            &EquationElements::Text(ref s) => s,
            &EquationElements::Symb(ref s) => s.get_string(),
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
        write!(&mut buf, r"\begin{{equation}}\n",).unwrap();
        for item in self.iter() {
            item.write_to_buffer(&mut buf);
        }
        write!(&mut buf, r"\n\end{{equation}}\n",).unwrap();
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
