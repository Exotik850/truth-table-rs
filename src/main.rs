use std::{
    collections::{HashMap, VecDeque},
    iter::Peekable,
    str::Chars,
};
#[cfg(test)]
mod test;

fn main() {
    let source = "a & b | c";
    let parser = FormulaParser::new(source);
    let formula = parser.parse();

    for node in formula.root.children() {
        println!("{:?}", node);
    }

    println!("{:?}", formula);

    let vars = [("a", true), ("b", false), ("c", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    println!("{}", formula.eval(&vars));
}

pub fn shunting_yard(input: &str) -> Vec<char> {
    let mut output = Vec::with_capacity(input.len());
    let mut stack: Vec<Operator> = Vec::new();
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
                    output.push(top.to_char());
                }
            }
            c => {
                let Some(o) = Operator::from_peekable(&mut input) else {
                    output.push(c);
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
                    output.push(stack.pop().unwrap().to_char());
                }
                stack.push(o);
            }
        }
        input.next();
    }
    output.extend(stack.drain(..).map(Operator::to_char).rev());
    output
}

// And, not, or, if, iff
#[derive(Debug)]
pub enum Node {
    And(Box<Node>, Box<Node>), // &
    Or(Box<Node>, Box<Node>),  // |
    Not(Box<Node>),            // ~
    If(Box<Node>, Box<Node>),  // ->
    Iff(Box<Node>, Box<Node>), // <->
    Expr(Vec<Box<Node>>),      // Expression
    Atom(String),              // Variable
}

impl Node {
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

            Node::Expr(children) => {
                for child in children {
                    stack.push_back(child);
                }
            }
            Node::Atom(_) => {
                // stack.push_back(self);
            }
        }
    }

    fn children(&self) -> Vec<&Node> {
        let mut stack = VecDeque::new();
        self._children(&mut stack);
        stack.into_iter().collect()
    }
}

struct FormulaParser {
    source: Vec<char>, // Source that went through shunting yard
    pos: usize,
}
impl FormulaParser {
    pub fn new(source: &str) -> FormulaParser {
        let source = shunting_yard(source);
        println!("{:?}", source);
        FormulaParser { source, pos: 0 }
    }

    pub fn parse(mut self) -> Formula {
        let root = self.parse_expr();
        println!("{:?}", root);
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

    fn parse_expr(&mut self) -> Node {
        let mut stack = Vec::new();

        while let Some(&token) = self.source.get(self.pos) {
            self.pos += 1;

            match token {
                '&' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::And(Box::new(left), Box::new(right)));
                }
                '|' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::Or(Box::new(left), Box::new(right)));
                }
                '~' => {
                    let operand = stack.pop().unwrap();
                    stack.push(Node::Not(Box::new(operand)));
                }
                '-' => {
                    // if self.source[self.pos] != '>' {
                    //     panic!("Invalid token");
                    // }
                    // self.pos += 1;
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::If(Box::new(left), Box::new(right)));
                }
                '<' => {
                    // if &self.source[self.pos..self.pos+1] != ['-', '>'] {
                    //     panic!("Invalid tokens");
                    // }
                    // self.pos += 2;
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::Iff(Box::new(left), Box::new(right)));
                }
                _ => {
                    stack.push(Node::Atom(token.to_string()));
                }
            }
        }

        assert_eq!(stack.len(), 1, "Invalid expression");
        stack.pop().unwrap()
    }
}
#[derive(Debug)]
pub struct Formula {
    root: Node,
    variables: Vec<String>,
}

impl Formula {
    pub fn eval(&self, vars: &HashMap<String, bool>) -> bool {
        self.eval_node(&self.root, vars)
    }

    fn eval_node(&self, node: &Node, vars: &HashMap<String, bool>) -> bool {
        match node {
            Node::And(left, right) => self.eval_node(left, vars) && self.eval_node(right, vars),
            Node::Or(left, right) => self.eval_node(left, vars) || self.eval_node(right, vars),
            Node::Not(operand) => !self.eval_node(operand, vars),
            Node::If(left, right) => !self.eval_node(left, vars) || self.eval_node(right, vars),
            Node::Iff(left, right) => self.eval_node(left, vars) == self.eval_node(right, vars),
            Node::Atom(s) => *vars.get(s).expect("Invalid variable"),
            Node::Expr(_) => panic!("Invalid node"),
        }
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
                if let Some('>') = input.peek() {
                    input.next();
                    Some(Operator::If)
                } else {
                    None
                }
            }
            '<' => {
                input.next();
                if let Some('-') = input.peek() {
                    input.next();
                    if let Some('>') = input.peek() {
                        input.next();
                        Some(Operator::Iff)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => Operator::from_char(c),
        }
    }

    fn from_char(c: char) -> Option<Operator> {
        match c {
            '&' => Some(Operator::And),
            '|' => Some(Operator::Or),
            '~' => Some(Operator::Not),
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
