use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

mod lexer;
mod operator;
mod parser;

#[cfg(test)]
mod test;

type NodeChild = Box<Node>;

pub use parser::FormulaParser;

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

impl From<&str> for Formula {
    fn from(s: &str) -> Self {
        let parser = parser::FormulaParser::new(s);
        parser.parse()
    }
}

#[derive(Debug)]
pub struct Formula {
    root: Node,
    variables: HashSet<String>,
}

fn eval_node(node: &Node, vars: &HashMap<String, bool>) -> Option<bool> {
    let out = match node {
        Node::And(left, right) => eval_node(left, vars)? && eval_node(right, vars)?,
        Node::Or(left, right) => eval_node(left, vars)? || eval_node(right, vars)?,
        Node::Not(operand) => !eval_node(operand, vars)?,
        Node::If(left, right) => !eval_node(left, vars)? || eval_node(right, vars)?,
        Node::Iff(left, right) => eval_node(left, vars)? == eval_node(right, vars)?,
        Node::Atom(s) => return vars.get(s).copied(),
    };
    Some(out)
}

impl Formula {
    pub fn eval(&self, vars: &HashMap<String, bool>) -> Option<bool> {
        eval_node(&self.root, vars)
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
            let result_str = match self.eval(&vars) {
                Some(op) => {
                    if op {
                        "T"
                    } else {
                        "F"
                    }
                }
                None => "E",
            };
            println!("{:^w$} |", result_str, w = root_str.len());
        }

        println!("{line}");
        println!("T: True, F: False");
    }
}
