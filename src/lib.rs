use std::rc::Rc;

mod simplifier;
mod tokenizer;

#[derive(Debug)]
pub enum Token {
    Num(f64),                  // A Number
    Add(Vec<Rc<Token>>),       // Addition
    Mul(Vec<Rc<Token>>),       // Multiplication
    Div(Rc<Token>, Rc<Token>), // Division
    Exp(Rc<Token>, Rc<Token>), // Exponentiation
    Rot(Rc<Token>, Rc<Token>), // Nth root
    Inc,                       // Unkown value "x"
}

#[derive(Debug)]
pub struct Operation<'a> {
    tokens: Rc<Token>,
    steps: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use crate::simplifier;
    use crate::tokenizer;

    #[test]
    fn simplify_test() {
        println!(
            "{:?}",
            simplifier::simplify(tokenizer::tokenize(
                r#"
                Mul(
                    Num(2),
                    Inc,
                    Num(3),
                    Num(4)
                )"#
            ))
        );
    }
}
