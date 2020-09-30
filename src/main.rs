mod arithmetic;
mod stack;

use crate::arithmetic::{ArithmeticError, Number, Token, Precedence};
use crate::stack::Stack;
use std::borrow::BorrowMut;
use std::env;
use std::str::Chars;

const TEN: Number = 10.0;

fn tokenize_number(cursor: &mut Option<char>, characters: &mut Chars) -> Option<Token> {
    let mut result: Number = 0.0;
    let mut place: i32 = 0;
    let mut radix: bool = false;

    if cursor.is_none() {
        return None
    }

    while let Some(character) = cursor {
         match character {
            '0'..='9' => {
                result *= TEN;
                result += (*character as u32 - '0' as u32) as Number;
                if radix {
                    place += 1;
                }
            }
            '.' => radix = true,
            _ => break,
        }
        *cursor = characters.next();
    }

    if radix {
        result /= TEN.powi(place);
    }
    Some(Token::Number(result))
}

fn tokenize_symbol(cursor: &mut Option<char>, characters: &mut Chars) -> Option<Token> {
    let result = match cursor {
        None => return None,
        Some(character) => match character {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Times),
            '/' => Some(Token::Divide),
            '(' => Some(Token::Left),
            ')' => Some(Token::Right),
            _ => None,
        },
    };
    *cursor = characters.next();
    result
}

fn tokenize(expression: &String) -> Result<Stack<Token>, ArithmeticError> {
    let mut result: Stack<Token> = Stack::new();
    let mut characters = expression.chars();
    let mut cursor: Option<char> = characters.next();
    while let Some(character) = cursor {
        match character {
            '0' ..= '9' => result.push(tokenize_number(cursor.borrow_mut(), characters.borrow_mut()).unwrap()),
            '+' | '-' | '*' | '/' | '(' | ')' => result.push(tokenize_symbol(cursor.borrow_mut(), characters.borrow_mut()).unwrap()),
            ' ' | '\t' | '\n' | '\r' => cursor = characters.borrow_mut().next(),
            _ => return Err(ArithmeticError::new(format!("invalid character {}", character).as_str())),
        }
    }
    Ok(result.reversed())
}

fn evaluate_primary(tokens: &mut Stack<Token>) -> Result<Number, ArithmeticError> {
    let top_token = tokens.pop();
    match top_token {
        Some(Token::Left) => {
            let result = evaluate(tokens)?;
            match tokens.pop() {
                Some(Token::Right) => Ok(result),
                _ => Err(ArithmeticError::new("expected closing paren")),
            }
        },
        Some(Token::Minus) => Ok(-evaluate_primary(tokens)?),
        Some(Token::Number(value)) => Ok(value),
        _ => Err(ArithmeticError::new("unexpected token")),
    }
}

pub fn precedence_if_reducible(tokens: &mut Stack<Token>, minimum_precedence: Precedence) -> Option<Precedence> {
    match tokens.peek() {
        None => None,
        Some(token) => {
            let precedence = token.precedence();
            if token.binary() && precedence > minimum_precedence {
                Some(precedence)
            } else {
                None
            }
        },
    }
}

pub fn pop_if_reducible(tokens: &mut Stack<Token>, minimum_precedence: Precedence) -> Option<Token> {
    let peek = tokens.peek();
    match peek {
        None => None,
        Some(token) => {
            if token.binary() && token.precedence() >= minimum_precedence {
                tokens.pop()
            } else {
                None
            }
        },
    }
}

fn evaluate_expression(tokens: &mut Stack<Token>, mut left: Number, minimum_precedence: Precedence) -> Result<Number, ArithmeticError> {
    while let Some(operator) = pop_if_reducible(tokens, minimum_precedence) {
        let mut right = evaluate_primary(tokens)?;

        while let Some(new_minimum_precedence) = precedence_if_reducible(tokens, operator.precedence()) {
            right = evaluate_expression(tokens, right, new_minimum_precedence)?;
        }

        left = operator.binary_apply(left, right)?;
    }
    Ok(left)
}

fn evaluate(tokens: &mut Stack<Token>) -> Result<Number, ArithmeticError> {
    let left = evaluate_primary(tokens)?;
    let result = evaluate_expression(tokens, left, 0);
    if result.is_ok() && tokens.peek().is_some() {
        Err(ArithmeticError::new("not all of expression was parsed"))
    } else {
        result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("expected expression!");
        return;
    }

    let expression: &String = &args[1];
    let tokenize_result = tokenize(expression);
    if tokenize_result.is_err() {
        eprintln!("{}", tokenize_result.err().unwrap());
        return;
    }

    let mut tokens = tokenize_result.unwrap();
    let evaluate_result = evaluate(tokens.borrow_mut());
    if evaluate_result.is_err() {
        eprintln!("{}", evaluate_result.err().unwrap());
        return;
    }

    println!("{}", evaluate_result.unwrap());
}
