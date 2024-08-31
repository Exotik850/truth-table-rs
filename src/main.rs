use std::{collections::HashMap, iter::Peekable, str::Chars};
fn main() {
    let source = "(~p & q) | (p & ~q) | (p & q)";
    let parser = FormulaParser::new(source);
    let formula = parser.parse();
    println!("{:?}", formula);
}
pub fn shunting_yard(input: &str) -> Vec<char> {
    let mut output = Vec::with_capacity(input.len());
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            ' ' => continue,
            '(' => stack.push(c),
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    output.push(top);
                }
            }
            c => {
                let Some(o) = Operator::from_char(c) else {
                    output.push(c);
                    continue;
                };
                while let Some(&top) = stack.last() {
                    if top == '(' {
                        break;
                    }
                    let Some(top_op) = Operator::from_char(top) else {
                        break;
                    };
                    if top_op.precedence() <= o.precedence()
                        && (top_op.precedence() != o.precedence()
                            || o.associativity() == Associativity::Right)
                    {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.push(c);
            }
        }
    }
    output.extend(stack.drain(..).rev());
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
    Expr(Vec<Node>),           // Expression
    Atom(String),              // Variable
}

struct NodeIter<'a> {
    node: &'a Node,
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            self.stack.push(self.node);
        }

        while let Some(node) = self.stack.pop() {
            match node {
                Node::And(left, right)
                | Node::Or(left, right)
                | Node::If(left, right)
                | Node::Iff(left, right) => {
                    self.stack.push(left);
                    self.stack.push(right);
                }
                Node::Not(operand) => {
                    self.stack.push(operand);
                }
                Node::Expr(expr) => {
                    self.stack.extend(expr.iter());
                }
                Node::Atom(_) => {
                    return Some(node);
                }
            }
        }

        None
    }
}

impl Node {
    fn iter(&self) -> NodeIter {
        NodeIter {
            node: self,
            stack: Vec::new(),
        }
    }
}

struct FormulaParser {
    source: Vec<char>, // Source that went through shunting yard
    pos: usize,
}
impl FormulaParser {
    pub fn new(source: &str) -> FormulaParser {
        let source = shunting_yard(source);
        FormulaParser { source, pos: 0 }
    }

    pub fn parse(mut self) -> Formula {
        let root = self.parse_expr();
        Formula {
            variables: root
                .iter()
                .filter_map(|n| match n {
                    Node::Atom(s) => Some(s),
                    _ => None,
                })
                .cloned()
                .collect(),
            root,
        }
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
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::If(Box::new(left), Box::new(right)));
                }
                '<' => {
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

// impl Formula {
//     pub fn eval(&self, vars: &HashMap<String, bool>) -> bool {
//         self.eval_node(&self.root, vars)
//     }

//     fn eval_node(&self, node: &Node, vars: &HashMap<String, bool>) -> bool {
//         match node {
//             Node::And(left, right) => {
//                 self.eval_node(left 
// }

#[derive(Clone, Copy, Debug)]
enum Operator {
    And,
    Or,
    Not,
    If,
    Iff,
}

impl Operator {
    fn from_char(c: char) -> Option<Operator> {
        match c {
            '&' => Some(Operator::And),
            '|' => Some(Operator::Or),
            '~' => Some(Operator::Not),
            '-' => Some(Operator::If), // TODO allow for -> and <->
            '<' => Some(Operator::Iff),
            _ => None,
        }
    }

    fn precedence(self) -> u8 {
        match self {
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Associativity {
    Left,
    Right,
}
