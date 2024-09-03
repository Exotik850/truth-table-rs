use std::{iter::Peekable, str::Chars};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Not,
    If,
    Iff,
    Parenthesis,
}

impl Operator {
    pub fn from_peekable(input: &mut Peekable<Chars>) -> Option<Operator> {
        let &c = input.peek()?;
        match c {
            '-' => {
                input.next();
                let Some('>') = input.peek() else { return None };
                input.next();
                Some(Operator::If)
            }
            '<' => {
                input.next();
                let Some('-') = input.peek() else { return None };
                input.next();
                let Some('>') = input.peek() else { return None };
                input.next();
                Some(Operator::Iff)
            }
            '&' => {
                input.next();
                if let Some('&') = input.peek() {
                    input.next();
                }
                Some(Operator::And)
            }
            '|' => {
                input.next();
                if let Some('|') = input.peek() {
                    input.next();
                }
                Some(Operator::Or)
            }
            _ => Operator::from_char(c),
        }
    }

    fn from_char(c: char) -> Option<Operator> {
        match c {
            '&' => Some(Operator::And),
            '|' => Some(Operator::Or),
            '~' | '!' => Some(Operator::Not),
            '(' | ')' => Some(Operator::Parenthesis),
            _ => None,
        }
    }

    pub fn precedence(self) -> u8 {
        match self {
            Operator::Parenthesis => 4,
            Operator::Not => 3,
            Operator::And => 2,
            Operator::Or => 1,
            Operator::If | Operator::Iff => 0,
        }
    }

    pub fn associativity(self) -> Associativity {
        match self {
            Operator::Not => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}
