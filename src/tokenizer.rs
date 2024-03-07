use std::rc::Rc;

use crate::{Operation, Token};

const TOKEN_LEN: usize = 3;

pub fn tokenize(operation: &str) -> Operation {
    let operation: String = operation.chars().filter(|&c| !c.is_whitespace()).collect();

    Operation {
        tokens: Rc::new(tokenize_operator(&operation)),
        steps: Vec::new(),
    }
}

fn tokenize_operator(operation: &str) -> Token {
    let operator = &operation[0..TOKEN_LEN];
    let content = &operation[TOKEN_LEN..];
    match operator {
        "Num" => {
            let num = content[1..content.len() - 1]
                .parse::<f64>()
                .expect("Number format was invalid: EXPECTED '1.2'");
            return Token::Num(num);
        }
        "Add" => {
            let (first, second) = split_token(content);
            return Token::Add(Rc::new(first), Rc::new(second));
        }
        "Sub" => {
            let (first, second) = split_token(content);
            return Token::Sub(Rc::new(first), Rc::new(second));
        }
        "Mul" => {
            let (first, second) = split_token(content);
            return Token::Mul(Rc::new(first), Rc::new(second));
        }
        "Div" => {
            let (first, second) = split_token(content);
            return Token::Div(Rc::new(first), Rc::new(second));
        }
        "Exp" => {
            let (first, second) = split_token(content);
            return Token::Exp(Rc::new(first), Rc::new(second));
        }
        "Rot" => {
            let (first, second) = split_token(content);
            return Token::Rot(Rc::new(first), Rc::new(second));
        }
        other => panic!("Operator not known: {}", other),
    }
}

fn split_token(content: &str) -> (Token, Token) {
    let content = &content[1..content.len() - 1];
    let (first, second) = split_token_string(content);

    let first_token = tokenize_operator(first);
    let second_token = tokenize_operator(second);

    (first_token, second_token)
}

fn split_token_string(content: &str) -> (&str, &str) {
    let mut depth: u32 = 0;
    let mut idx = 0;
    for (index, char) in content.chars().enumerate() {
        match char {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' => {
                if depth == 0 {
                    idx = index;
                    break;
                }
            }
            _ => (),
        }
    }

    (&content[0..idx], &content[idx + 1..])
}
