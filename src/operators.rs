/// File defining the structure of an operator
///
use std::io::BufWriter;
use std::io::Write;
use std::fmt;
use core::*;
use latex_file::*;
use equations::*;

pub enum Operators<T, A, B> {
    /// Sum(variable, begin, end) body
    Sum(T, A, B),
    /// Product(variable, begin, end) body
    Product(T, A, B),
}

impl<T: fmt::Display, A: fmt::Display, B: fmt::Display> Operators<T, A, B> {
    /// Returns the Latex Code for the operator
    pub fn latex_code(&self) -> &str {
        let code = match self {
            &Operators::Sum(_, _, _) => "\\sum",
            &Operators::Product(_, _, _) => "\\prod",
        };
        code
    }

    /// Returns the variable of the operator
    pub fn variable(&self) -> &T {
        match self {
            &Operators::Sum(ref v, _, _) => v,
            &Operators::Product(ref v, _, _) => v,
        }
    }

    /// Returns the variable of the operator
    pub fn begin(&self) -> &A {
        match self {
            &Operators::Sum(_, ref b, _) => b,
            &Operators::Product(_, ref b, _) => b,
        }
    }

    /// Returns the variable of the operator
    pub fn end(&self) -> &B {
        match self {
            &Operators::Sum(_, _, ref e) => e,
            &Operators::Product(_, _, ref e) => e,
        }
    }
}

impl<T: fmt::Display, A: fmt::Display, B: fmt::Display> Writable for Operators<T, A, B> {
    fn write_latex(&self, mut file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(
            &mut buf,
            "{}_{{{} = {}}}^{{{}}}",
            self.latex_code(),
            self.variable(),
            self.begin(),
            self.end(),
        ).unwrap()
    }
}

#[cfg(test)]
mod tests_operators {
    use super::*;
    use equations::new_equation;

    #[test]
    fn test_sum() {
        let mut f = new_latex_file("./tests_results/operators/sum.tex");
        f.begin_document();
        let s = Operators::Sum("i", 0, "n");
        s.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_prod() {
        let mut f = new_latex_file("./tests_results/operators/prod.tex");
        f.begin_document();
        let s = Operators::Product("i", 0, "n");
        s.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_sum_with_equation() {
        let mut f = new_latex_file("./tests_results/operators/sum_with_equation.tex");
        f.begin_document();
        let s = Operators::Sum("i", 0, "n");
        let eq = new_equation(&vec!["i"]);
        eq.push(EquationElements::Operator(s));
        eq.write_latex(&mut f);
        f.write_footer();
    }

}
