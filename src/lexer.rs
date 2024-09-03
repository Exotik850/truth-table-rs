use crate::operator::{Associativity, Operator};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Operator(Operator),
    Atom(String),
    Value(bool),
}

pub fn shunting_yard(input: &str) -> Vec<Token> {
    let mut output = Vec::with_capacity(input.len());
    let mut stack: Vec<Operator> = Vec::new();
    let mut current_atom = String::new();
    let mut input = input.chars().peekable();
    while let Some(&c) = input.peek() {
        match c {
            ' ' => {
                input.next();
                continue;
            }
            '(' => stack.push(Operator::Parenthesis),
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == Operator::Parenthesis {
                        break;
                    }
                    output.push(Token::Operator(top));
                }
            }
            't' | 'T' | 'f' | 'F' => {
              // Check if it contains the words "true" or "false"
                let mut value = String::new();
                while let Some(&c) = input.peek() {
                    if c.is_alphabetic() {
                        value.push(c);
                        input.next();
                    } else {
                        break;
                    }
                }
                value.make_ascii_lowercase();
                output.push(Token::Value(value == "true" || value == "t"));
                continue;
            }
            c if c.is_alphabetic() => {
                while let Some(&c) = input.peek() {
                    if c.is_alphabetic() {
                        current_atom.push(c);
                        input.next();
                    } else {
                        break;
                    }
                }
                output.push(Token::Atom(std::mem::take(&mut current_atom)));
                continue;
            }
            c => {
                let Some(o) = Operator::from_peekable(&mut input) else {
                    output.push(Token::Atom(c.to_string()));
                    input.next();
                    continue;
                };
                while let Some(&top) = stack.last() {
                    if top == Operator::Parenthesis {
                        break;
                    }
                    if top.precedence() <= o.precedence()
                        && (top.precedence() != o.precedence()
                            || o.associativity() == Associativity::Right)
                    {
                        break;
                    }
                    output.push(Token::Operator(stack.pop().unwrap()));
                }
                stack.push(o);
            }
        }
        input.next();
    }
    output.extend(stack.drain(..).map(Token::Operator).rev());
    output
}
