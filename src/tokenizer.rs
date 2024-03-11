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
            let tokens = split_variable_token(content);
            return Token::Add(tokens);
        }
        "Mul" => {
            let tokens = split_variable_token(content);
            return Token::Mul(tokens);
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
        "Inc" => return Token::Inc,
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

fn split_variable_token(content: &str) -> Vec<Rc<Token>> {
    let mut tokens = Vec::new();
    let content = &content[1..content.len() - 1];

    let args = split_variable_token_string(content);

    for arg in args {
        tokens.push(Rc::new(tokenize_operator(arg)));
    }

    tokens
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

fn split_variable_token_string(content: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    let mut depth: u32 = 0;
    let mut prev_idx = 0;
    for (index, char) in content.chars().enumerate() {
        match char {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' => {
                if depth == 0 {
                    tokens.push(&content[prev_idx..index]);
                    prev_idx = index + 1;
                }
            }
            _ => (),
        }
    }
    tokens.push(&content[prev_idx..]);

    tokens
}
