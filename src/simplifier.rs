use std::rc::Rc;

use crate::{Operation, Token};

pub fn simplify(operation: Operation) -> Operation {
    let mut steps = operation.steps;

    steps.push("Simplify");

    Operation {
        tokens: simplify_expresion(operation.tokens),
        steps,
    }
}

fn simplify_expresion(expresion: Rc<Token>) -> Rc<Token> {
    match *expresion {
        Token::Num(_) => return expresion,
        Token::Add(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(a_value + b_value));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
        Token::Sub(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(a_value - b_value));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
        Token::Mul(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(a_value * b_value));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
        Token::Div(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(a_value / b_value));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
        Token::Exp(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(a_value.powf(b_value)));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
        Token::Rot(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            if let Token::Num(a_value) = *a {
                if let Token::Num(b_value) = *b {
                    return Rc::new(Token::Num(b_value.powf(1.0 / a_value)));
                }
            }
            return Rc::new(Token::Add(a, b));
        }
    }
}
