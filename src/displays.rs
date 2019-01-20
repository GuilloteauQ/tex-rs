/// File for displaying elements in LaTeX
///

use str_or_string::*;

/// Returns the \frac of the two arguments
pub fn frac<A: StrOrString, B: StrOrString>(top: A, bottom: B) -> String {
    format!("\\frac{{{}}}{{{}}}", top.convert_string(), bottom.convert_string())
}
