/// File to define symbols in LaTeX
///

#[derive(Debug, PartialEq, Clone)]
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

pub fn is_op(s: &String) -> bool {
    s == "==" || s == "=" || s == "<" || s == "<=" || s == ">" || s == ">=" || s == "!="
        || s == "<>"
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


