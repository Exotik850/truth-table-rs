use crate::{
    lexer::{shunting_yard, Token},
    operator::Operator,
    Formula, Node,
};

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
        }
        assert_eq!(stack.len(), 1, "Invalid expression");
        stack.pop().unwrap()
    }
}
