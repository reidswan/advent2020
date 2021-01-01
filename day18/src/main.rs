use common::{load_vec, take_first_number};
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let input: Vec<String> = load_vec("input/day18.txt");
    println!(
        "Part 1: {}",
        evaluate_and_sum(&input, MathType::Simple).unwrap()
    );
    println!(
        "Part 2: {}",
        evaluate_and_sum(&input, MathType::Advanced).unwrap()
    );
}

fn evaluate_and_sum(input: &[String], math_type: MathType) -> Result<usize, String> {
    let mut sum = 0;
    for line in input {
        let tokens = lex(&mut line.chars().peekable())?;
        let expression = Expression::parse(&mut tokens.iter().peekable(), math_type)?;
        sum += evaluate(&expression)
    }
    Ok(sum)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MathType {
    Simple,
    Advanced,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum BinOp {
    Plus,
    Times,
}

impl BinOp {
    fn operate(&self, l: usize, r: usize) -> usize {
        match self {
            BinOp::Plus => l + r,
            BinOp::Times => l * r,
        }
    }

    fn binding_power(&self, math_type: MathType) -> (usize, usize) {
        if math_type == MathType::Simple {
            // everything has the same precedence, and should be left-associative
            (1, 2)
        } else {
            match self {
                BinOp::Plus => (3, 4),
                BinOp::Times => (1, 2),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    BinOp {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    Number(usize),
}

impl Expression {
    fn parse<'a, T>(token_stream: &mut Peekable<T>, math_type: MathType) -> Result<Self, String>
    where
        T: Iterator<Item = &'a Token>,
    {
        Self::parse_binding_power(token_stream, 0, math_type)
    }

    // pratt parsing courtesy of https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    fn parse_binding_power<'a, T>(
        token_stream: &mut Peekable<T>,
        min_bp: usize,
        math_type: MathType,
    ) -> Result<Self, String>
    where
        T: Iterator<Item = &'a Token>,
    {
        let mut lhs = match token_stream.next().ok_or(String::from("Unexpected EOF"))? {
            Token::Number(n) => Ok(Expression::Number(*n)),
            Token::LeftParen => {
                let lhs = Self::parse_binding_power(token_stream, 0, math_type)?;
                if let Some(Token::RightParen) = token_stream.next() {
                    Ok(lhs)
                } else {
                    Err("Expected a closing right paren".into())
                }
            }
            t => Err(format!("Unexpected token at start of expr: {:?}", t)),
        }?;

        loop {
            let op = match token_stream.peek() {
                None | Some(Token::RightParen) => break,
                Some(Token::BinOp(b)) => *b,
                Some(t) => return Err(format!("Unexpected token {:?}", t)),
            };

            let (l_bp, r_bp) = op.binding_power(math_type);
            if l_bp < min_bp {
                break;
            }

            token_stream.next();

            let right = Box::new(Self::parse_binding_power(token_stream, r_bp, math_type)?);

            lhs = Expression::BinOp {
                left: Box::new(lhs),
                op,
                right,
            }
        }

        Ok(lhs)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(usize),
    LeftParen,
    RightParen,
    BinOp(BinOp),
}

impl Token {
    fn from_char(c: char) -> Option<Token> {
        Some(match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' => Token::BinOp(BinOp::Plus),
            '*' => Token::BinOp(BinOp::Times),
            _ => return None,
        })
    }
}

fn lex(src: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    while let Some(&c) = src.peek() {
        match c {
            '(' | ')' | '+' | '*' => {
                src.next();
                tokens.push(Token::from_char(c).unwrap());
            }
            c if c.is_ascii_digit() => tokens.push(Token::Number(take_first_number(src)?)),
            c if c.is_ascii_whitespace() => {
                src.next();
            }
            _ => return Err(format!("Unrecognized token while lexing: '{}'", c)),
        }
    }

    Ok(tokens)
}

fn evaluate(expression: &Expression) -> usize {
    match expression {
        Expression::Number(n) => *n,
        Expression::BinOp { left, op, right } => op.operate(evaluate(left), evaluate(right)),
    }
}
