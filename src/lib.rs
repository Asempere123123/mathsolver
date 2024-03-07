use std::rc::Rc;

mod simplifier;
mod tokenizer;

#[derive(Debug)]
pub enum Token {
    Num(f64),                  // A Number
    Add(Rc<Token>, Rc<Token>), // Addition
    Sub(Rc<Token>, Rc<Token>), // Substraction
    Mul(Rc<Token>, Rc<Token>), // Multiplication
    Div(Rc<Token>, Rc<Token>), // Division
    Exp(Rc<Token>, Rc<Token>), // Exponentiation
    Rot(Rc<Token>, Rc<Token>), // Nth root
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
                Add(
                    Exp(
                        Num(2),
                        Num(3)
                    ),
                    Mul(
                        Num(2),
                        Num(3)
                    )
                )"#
            ))
        );
    }
}
