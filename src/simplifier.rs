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
        Token::Add(ref tokens) => {
            let mut simplified_tokens = Vec::new();

            // Es un Rc clone
            for token in tokens {
                simplified_tokens.push(simplify_expresion(token.clone()));
            }

            let mut count = 0.0;
            let mut indices = Vec::new();
            for (index, token) in simplified_tokens.iter().enumerate() {
                if let Token::Num(value) = **token {
                    count += value;
                    indices.push(index);
                }
            }
            for i in indices.iter().rev() {
                simplified_tokens.remove(*i);
            }

            if count != 0.0 {
                simplified_tokens.push(Rc::new(Token::Num(count)));
            }

            if simplified_tokens.len() == 1 {
                if let Token::Num(value) = *simplified_tokens[0] {
                    return Rc::new(Token::Num(value));
                }
            }

            Rc::new(Token::Add(simplified_tokens))
        }
        Token::Mul(ref tokens) => {
            let mut simplified_tokens = Vec::new();

            // Es un Rc clone
            for token in tokens {
                simplified_tokens.push(simplify_expresion(token.clone()));
            }

            let mut count = 1.0;
            let mut indices = Vec::new();
            for (index, token) in simplified_tokens.iter().enumerate() {
                if let Token::Num(value) = **token {
                    count *= value;
                    indices.push(index);
                }
            }
            for i in indices.iter().rev() {
                simplified_tokens.remove(*i);
            }

            if count != 0.0 {
                simplified_tokens.push(Rc::new(Token::Num(count)));
            }

            if simplified_tokens.len() == 1 {
                if let Token::Num(value) = *simplified_tokens[0] {
                    return Rc::new(Token::Num(value));
                }
            }

            Rc::new(Token::Mul(simplified_tokens))
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
            return Rc::new(Token::Div(a, b));
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
            return Rc::new(Token::Exp(a, b));
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
            return Rc::new(Token::Rot(a, b));
        }
        Token::Inc => return expresion,
        Token::Equ(ref a, ref b) => {
            // Es un Rc clone
            let a = simplify_expresion(a.clone());
            let b = simplify_expresion(b.clone());

            return Rc::new(Token::Equ(a, b));
        }
    }
}
