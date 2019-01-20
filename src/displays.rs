/// File for displaying elements in LaTeX
///


/// Returns the \frac of the two arguments
pub fn frac<A: AsRef<str>, B: AsRef<str>>(top: A, bottom: B) -> String {
    format!("\\frac{{{}}}{{{}}}", top.as_ref().to_string(), bottom.as_ref().to_string())
}
