/// Implements a Trait to have String and &str

pub trait StrOrString {
    fn convert_string(&self) -> String;
}

impl<'a> StrOrString for &'a str {
    fn convert_string(&self) -> String {
        String::from(*self)
    }
}

impl StrOrString for String {
    fn convert_string(&self) -> String {
        let mut result = String::new();
        for letter in (*self).chars() {
            result.push(letter);
        }
        result
    }
}
