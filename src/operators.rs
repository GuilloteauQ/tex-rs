/// File defining the structure of an operator
///
use std::io::BufWriter;
use std::io::Write;
use latex_file::*;
use writable::*;


#[derive(Clone)]
pub enum VarOrImm {
    Var(String),
    Imm(i32),
}

pub trait VarOrImmTrait {
    fn to_var_or_imm(&self) -> VarOrImm;
}

impl<'a> VarOrImmTrait for &'a str {
    fn to_var_or_imm(&self) -> VarOrImm {
        VarOrImm::Var(String::from(*self))
    }
}

impl VarOrImmTrait for i32 {
    fn to_var_or_imm(&self) -> VarOrImm {
        VarOrImm::Imm(*self)
    }
}

impl VarOrImm {
    pub fn new<T: VarOrImmTrait>(v: T) -> Self {
        v.to_var_or_imm()
    }

    pub fn get_val(&self) -> String {
        match self {
            &VarOrImm::Var(ref x) => String::from(format!("{}", x)),
            &VarOrImm::Imm(ref x) => String::from(format!("{}", x)),
        }
    }
}

#[derive(Clone)]
pub enum Operators {
    /// Sum(variable, begin, end) body
    Sum(VarOrImm, VarOrImm, VarOrImm),
    /// Product(variable, begin, end) body
    Product(VarOrImm, VarOrImm, VarOrImm),
}

impl Operators {
    /// Returns the Latex Code for the operator
    pub fn latex_code_op(&self) -> &str {
        let code = match self {
            &Operators::Sum(_, _, _) => "\\sum",
            &Operators::Product(_, _, _) => "\\prod",
        };
        code
    }

    /// Returns the variable of the operator
    pub fn variable(&self) -> &VarOrImm {
        match self {
            &Operators::Sum(ref v, _, _) => v,
            &Operators::Product(ref v, _, _) => v,
        }
    }

    /// Returns the variable of the operator
    pub fn begin(&self) -> &VarOrImm {
        match self {
            &Operators::Sum(_, ref b, _) => b,
            &Operators::Product(_, ref b, _) => b,
        }
    }

    /// Returns the variable of the operator
    pub fn end(&self) -> &VarOrImm {
        match self {
            &Operators::Sum(_, _, ref e) => e,
            &Operators::Product(_, _, ref e) => e,
        }
    }

    /// Returns the LaTex String for the operator
    pub fn latex_code(&self) -> String {
        format!(
            "{} _{{{} = {}}} ^{{{}}}",
            self.latex_code_op(),
            self.variable().get_val(),
            self.begin().get_val(),
            self.end().get_val(),
        )
    }
}

impl Writable for Operators {
    fn write_latex(&self, file: &mut LatexFile) {
        let mut writer = BufWriter::new(file);
        self.write_to_buffer(&mut writer);
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        write!(&mut buf, "{}", self.latex_code(),).unwrap()
    }
}

#[cfg(test)]
mod tests_operators {
    use super::*;
    use equations::*;

    #[test]
    fn test_sum() {
        let mut f = new_latex_file("./tests_results/operators/sum.tex");
        f.begin_document();
        let s = Operators::Sum(VarOrImm::new("i"), VarOrImm::new(0), VarOrImm::new("n"));
        s.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_prod() {
        let mut f = new_latex_file("./tests_results/operators/prod.tex");
        f.begin_document();
        let s = Operators::Product(VarOrImm::new("i"), VarOrImm::new(0), VarOrImm::new("n"));
        s.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_sum_with_equation() {
        let mut f = new_latex_file("./tests_results/operators/sum_with_equation.tex");
        f.begin_document();
        let s = Operators::Sum(VarOrImm::new("i"), VarOrImm::new(0), VarOrImm::new("n"));
        let mut eq = new_equation(&vec!["x", "="]);
        eq.push(EquationElements::Operator(s));
        eq.write_latex(&mut f);
        f.write_footer();
    }

}
