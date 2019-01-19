/// File for displaying elements in LaTeX
///

use str_or_string::*;

pub fn frac<A: StrOrString, B: StrOrString>(top: A, bottom: B) -> String {
    format!("\\frac{{{}}}{{{}}}", top.convert_string(), bottom.convert_string())
}
