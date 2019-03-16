/// File for displaying elements in LaTeX
///

/// Returns the \frac of the two arguments
pub fn frac<A: AsRef<str>, B: AsRef<str>>(top: A, bottom: B) -> String {
    format!(
        "\\frac{{{}}}{{{}}}",
        top.as_ref().to_string(),
        bottom.as_ref().to_string()
    )
}

/// Texttt for a text
pub fn texttt<T: AsRef<str>>(text: T) -> String {
    format!("\\texttt{{{}}}", text.as_ref().to_string())
}

/*
/// Single param
/// \name{arg}
pub fn single_arg<A: AsRef<str>, B: AsRef<str>>(name: A, arg: B) -> String {
    format!(
        "\\{}{{{}}}",
        name.as_ref().to_string(),
        arg.as_ref().to_string()
    )
}
*/

/// Single param
/// \name{arg}
// pub fn single_arg<A, F>(name: A) -> Box<F>
// where
//     A: AsRef<str>,
//     F: Fn(String) -> String,
// {
//     Box::new(move |arg: String| format!("\\{}{{{}}}", name.as_ref().to_string(), arg))
//         as Box<Fn(String) -> String>
// }
//
/// Two params
/// \name[arg1]{arg2}
pub fn two_args<A: AsRef<str>, B: AsRef<str>, C: AsRef<str>>(name: A, arg1: B, arg2: C) -> String {
    format!(
        "\\{}[{}]{{{}}}",
        name.as_ref().to_string(),
        arg1.as_ref().to_string(),
        arg2.as_ref().to_string()
    )
}
