use std::{error::Error, fmt};

pub type Number = f32;

#[derive(Debug)]
pub struct ArithmeticError {
    what: String,
}

impl ArithmeticError {
    pub fn new(what: &str) -> Self {
        Self {
            what: String::from(what),
        }
    }
}

impl Error for ArithmeticError {}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

#[derive(Debug)]
pub enum Token {
    Number(Number),
    Plus,
    Minus,
    Times,
    Divide,
    Left,
    Right,
}

pub type Precedence = u8;

impl Token {
    pub fn precedence(&self) -> Precedence {
        match *self {
            Token::Times | Token::Divide => 2,
            Token::Plus | Token::Minus => 1,
            _ => 0,
        }
    }

    pub fn binary(&self) -> bool {
        match *self {
            Token::Plus | Token::Minus | Token::Times | Token::Divide => true,
            _ => false,
        }
    }

    pub fn binary_apply(&self, left: Number, right: Number) -> Result<Number, ArithmeticError> {
        match *self {
            Token::Plus => Ok(left + right),
            Token::Minus => Ok(left - right),
            Token::Times => Ok(left * right),
            Token::Divide => {
                if right != 0.0 {
                    Ok(left / right)
                } else {
                    Err(ArithmeticError::new("division by zero"))
                }
            }
            _ => Err(ArithmeticError::new("invalid binary operator")),
        }
    }

    pub fn unary(&self) -> bool {
        match *self {
            Token::Plus | Token::Minus => true,
            _ => false,
        }
    }

    pub fn unary_apply(&self, number: Number) -> Result<Number, ArithmeticError> {
        match *self {
            Token::Plus => Ok(number),
            Token::Minus => Ok(-number),
            _ => Err(ArithmeticError::new("invalid unary operator")),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Number(value) => write!(f, "{}", value),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Left => write!(f, "("),
            Token::Right => write!(f, ")"),
        }
    }
}
