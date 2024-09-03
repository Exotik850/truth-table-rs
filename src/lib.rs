use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    iter::Peekable,
    str::Chars,
};

#[cfg(test)]
mod test;

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

type NodeChild = Box<Node>;

// And, not, or, if, iff
#[derive(Debug)]
pub enum Node {
    And(NodeChild, NodeChild), // &
    Or(NodeChild, NodeChild),  // |
    Not(NodeChild),            // ~
    If(NodeChild, NodeChild),  // ->
    Iff(NodeChild, NodeChild), // <->
    Atom(String),              // Variable
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_precedence(f, 0)
    }
}
use std::fmt;
impl Node {
    fn precedence(&self) -> u8 {
        match self {
            Node::Atom(_) => 5,
            Node::Not(_) => 4,
            Node::And(_, _) => 3,
            Node::Or(_, _) => 2,
            Node::If(_, _) => 1,
            Node::Iff(_, _) => 0,
        }
    }

    fn fmt_with_precedence(
        &self,
        f: &mut fmt::Formatter<'_>,
        parent_precedence: u8,
    ) -> fmt::Result {
        let this_precedence = self.precedence();
        let need_parens = this_precedence < parent_precedence;

        if need_parens {
            write!(f, "(")?;
        }

        match self {
            Node::And(left, right) => {
                left.fmt_with_precedence(f, this_precedence)?;
                write!(f, " & ")?;
                right.fmt_with_precedence(f, this_precedence)?;
            }
            Node::Or(left, right) => {
                left.fmt_with_precedence(f, this_precedence)?;
                write!(f, " | ")?;
                right.fmt_with_precedence(f, this_precedence)?;
            }
            Node::Not(operand) => {
                write!(f, "~")?;
                operand.fmt_with_precedence(f, this_precedence)?;
            }
            Node::If(left, right) => {
                left.fmt_with_precedence(f, this_precedence)?;
                write!(f, " -> ")?;
                right.fmt_with_precedence(f, this_precedence)?;
            }
            Node::Iff(left, right) => {
                left.fmt_with_precedence(f, this_precedence)?;
                write!(f, " <-> ")?;
                right.fmt_with_precedence(f, this_precedence)?;
            }
            Node::Atom(s) => write!(f, "{}", s)?,
        }

        if need_parens {
            write!(f, ")")?;
        }

        Ok(())
    }

    fn _children<'a>(&'a self, stack: &mut VecDeque<&'a Node>) {
        match self {
            Node::And(left, right)
            | Node::If(left, right)
            | Node::Or(left, right)
            | Node::Iff(left, right) => {
                stack.push_back(left);
                left._children(stack);
                stack.push_back(right);
                right._children(stack);
            }
            Node::Not(operand) => {
                stack.push_back(operand);
                operand._children(stack);
            }
            _ => {}
        }
    }
    fn children(&self) -> Vec<&Node> {
        let mut stack = VecDeque::new();
        self._children(&mut stack);
        stack.into_iter().collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Operator(Operator),
    Atom(String),
}

pub struct FormulaParser {
    source: Vec<Token>, // Source that went through shunting yard
}
impl FormulaParser {
    pub fn new(source: &str) -> FormulaParser {
        let source = shunting_yard(source);
        // println!("{:?}", source);
        FormulaParser { source }
    }

    pub fn parse(self) -> Formula {
        let root = self.parse_expr();
        // println!("{:?}", root);
        let variables = root
            .children()
            .into_iter()
            .filter_map(|n| match n {
                Node::Atom(s) => Some(s),
                _ => None,
            })
            .cloned()
            .collect();
        Formula { variables, root }
    }

    fn parse_expr(self) -> Node {
        let mut stack = Vec::new();

        for token in self.source {
            let op = match token {
                Token::Atom(atom) => {
                    stack.push(Node::Atom(atom));
                    continue;
                }
                Token::Operator(op) => op,
            };

            match op {
                Operator::And => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::And(Box::new(left), Box::new(right)));
                }
                Operator::Or => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::Or(Box::new(left), Box::new(right)));
                }
                Operator::Not => {
                    let operand = stack.pop().unwrap();
                    stack.push(Node::Not(Box::new(operand)));
                }
                Operator::If => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::If(Box::new(left), Box::new(right)));
                }
                Operator::Iff => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::Iff(Box::new(left), Box::new(right)));
                }
                _ => panic!("Invalid token"),
            }

            // match token {
            //     '&' => {
            //         let right = stack.pop().unwrap();
            //         let left = stack.pop().unwrap();
            //         stack.push(Node::And(Box::new(left), Box::new(right)));
            //     }
            //     '|' => {
            //         let right = stack.pop().unwrap();
            //         let left = stack.pop().unwrap();
            //         stack.push(Node::Or(Box::new(left), Box::new(right)));
            //     }
            //     '~' => {
            //         let operand = stack.pop().unwrap();
            //         stack.push(Node::Not(Box::new(operand)));
            //     }
            //     '-' => {
            //         // if self.source[self.pos] != '>' {
            //         //     panic!("Invalid token");
            //         // }
            //         // self.pos += 1;
            //         let right = stack.pop().unwrap();
            //         let left = stack.pop().unwrap();
            //         stack.push(Node::If(Box::new(left), Box::new(right)));
            //     }
            //     '<' => {
            //         // if &self.source[self.pos..self.pos+1] != ['-', '>'] {
            //         //     panic!("Invalid tokens");
            //         // }
            //         // self.pos += 2;
            //         let right = stack.pop().unwrap();
            //         let left = stack.pop().unwrap();
            //         stack.push(Node::Iff(Box::new(left), Box::new(right)));
            //     }
            //     _ => {
            //         stack.push(Node::Atom(token.to_string()));
            //     }
            //     }
            // }
        }

        // println!("{:?}", stack);
        assert_eq!(stack.len(), 1, "Invalid expression");
        stack.pop().unwrap()
    }
}

#[derive(Debug)]
pub struct Formula {
    root: Node,
    variables: HashSet<String>,
}

impl Formula {
    pub fn eval(&self, vars: &HashMap<String, bool>) -> Option<bool> {
        self.eval_inner(&self.root, vars)
    }

    fn eval_inner(&self, node: &Node, vars: &HashMap<String, bool>) -> Option<bool> {
        let out = match node {
            Node::And(left, right) => {
                self.eval_inner(left, vars)? && self.eval_inner(right, vars)?
            }
            Node::Or(left, right) => {
                self.eval_inner(left, vars)? || self.eval_inner(right, vars)?
            }
            Node::Not(operand) => !self.eval_inner(operand, vars)?,
            Node::If(left, right) => {
                !self.eval_inner(left, vars)? || self.eval_inner(right, vars)?
            }
            Node::Iff(left, right) => {
                self.eval_inner(left, vars)? == self.eval_inner(right, vars)?
            }
            Node::Atom(s) => return vars.get(s).copied(),
        };
        Some(out)
    }

    pub fn print_truth_table(&self) {
        // For every combination of variables (true / false) print the result of the formula
        let mut vars = HashMap::new();
        let mut variables = self.variables.iter().collect::<Vec<_>>();
        variables.sort_unstable();

        // Print header
        print!("| ");
        for var in &variables {
            print!("{:^5} | ", var);
        }
        let root_str = format!("{}", self.root);
        println!("{} |", root_str);
        let line_len = 4 + root_str.len() + 8 * variables.len();
        let line = "-".repeat(line_len);
        println!("{line}");

        let num_vars = variables.len();
        let num_rows = 1 << num_vars;

        for i in (0..num_rows).rev() {
            print!("| ");
            for (j, var) in variables.iter().enumerate() {
                let value = (i >> (num_vars - 1 - j)) & 1 == 1;
                vars.insert(var.to_string(), value);
                let value_str = if value { "T" } else { "F" };
                print!("{:^5} | ", value_str);
            }

            // Evaluate and print result
            let result = self.eval(&vars).unwrap_or(false);
            let result_str = if result { "T" } else { "F" };
            println!("{:^w$} |", result_str, w = root_str.len());
        }

        println!("{line}");
        println!("T: True, F: False");

        // Print header
        // print!("| ");
        // for var in &variables {
        //     print!("{:^5} | ", var);
        // }
        // let root_str = format!("{}", self.root);
        // println!("{} |", root_str);
        // let line_len = 4 + root_str.len() + 8 * variables.len();
        // let line = "-".repeat(line_len);
        // println!("{line}");

        // let mut i = 1 << self.variables.len();
        // while i > 0 {
        //     for (j, var) in variables.clone().into_iter().enumerate() {
        //         vars.insert(var.clone(), (i >> j) & 1 == 1);
        //     }
        //     // println!("{:?} -> {}", vars, self.eval(&vars).unwrap_or(false));
        //     print!("| ");
        //     for var in variables.clone() {
        //         let value = if *vars.get(var).unwrap() { "T" } else { "F" };
        //         print!("{:^5} | ", value);
        //     }

        //     // Evaluate and print result
        //     let result = self.eval(&vars).unwrap_or(false);
        //     let result_str = if result { "T" } else { "F" };
        //     println!("{:^w$} |", result_str, w = root_str.len());
        //     i -= 1;
        // }
        // println!("{line}");
        // println!("T: True, F: False");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Not,
    If,
    Iff,
    Parenthesis,
}

impl Operator {
    fn from_peekable(input: &mut Peekable<Chars>) -> Option<Operator> {
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

    fn precedence(self) -> u8 {
        match self {
            Operator::Parenthesis => 4,
            Operator::Not => 3,
            Operator::And => 2,
            Operator::Or => 1,
            Operator::If | Operator::Iff => 0,
        }
    }

    fn associativity(self) -> Associativity {
        match self {
            Operator::Not => Associativity::Right,
            _ => Associativity::Left,
        }
    }

    fn to_char(self) -> char {
        match self {
            Operator::And => '&',
            Operator::Or => '|',
            Operator::Not => '~',
            Operator::If => '-',
            Operator::Iff => '<',
            Operator::Parenthesis => '(',
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Associativity {
    Left,
    Right,
}
