use std::rc::Rc;

use crate::tokenizer::Token;

pub struct Simplifier;

impl Simplifier {
    pub fn simplify(expresion: Rc<Token>) -> Rc<Token> {
        match *expresion {
            Token::Num(_) => return expresion,
            Token::Add(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(a_value + b_value));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
            Token::Sub(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(a_value - b_value));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
            Token::Mul(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(a_value * b_value));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
            Token::Div(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(a_value / b_value));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
            Token::Exp(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(a_value.powf(b_value)));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
            Token::Rot(ref a, ref b) => {
                // Es un Rc clone
                let a = Self::simplify(a.clone());
                let b = Self::simplify(b.clone());

                if let Token::Num(a_value) = *a {
                    if let Token::Num(b_value) = *b {
                        return Rc::new(Token::Num(b_value.powf(1.0 / a_value)));
                    }
                }
                return Rc::new(Token::Add(a, b));
            }
        }
    }
}
