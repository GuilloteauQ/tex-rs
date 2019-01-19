/// File for displaying elements in LaTeX
///

pub fn frac(top: &str, bottom: &str) -> String {
    format!("\\frac{{{}}}{{{}}}", top, bottom)
}
