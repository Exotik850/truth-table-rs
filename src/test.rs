use lexer::{shunting_yard, Token};
use operator::Operator;
use parser::FormulaParser;

use super::*;

// Test Shunting Yard algorithm
#[test]
fn test_shunting_yard() {
    // assert_eq!(shunting_yard("a & b | c"), vec!['a', 'b', '&', 'c', '|']);
    // assert_eq!(shunting_yard("a & (b | c)"), vec!['a', 'b', 'c', '|', '&']);
    // assert_eq!(shunting_yard("~a & b"), vec!['a', '~', 'b', '&']);

    assert_eq!(
        shunting_yard("a & b | c"),
        vec![
            Token::Atom("a".to_string()),
            Token::Atom("b".to_string()),
            Token::Operator(Operator::And),
            Token::Atom("c".to_string()),
            Token::Operator(Operator::Or),
        ]
    );

    assert_eq!(
        shunting_yard("a & (b | c)"),
        vec![
            Token::Atom("a".to_string()),
            Token::Atom("b".to_string()),
            Token::Atom("c".to_string()),
            Token::Operator(Operator::Or),
            Token::Operator(Operator::And),
        ]
    );

    assert_eq!(
        shunting_yard("~a & b"),
        vec![
            Token::Atom("a".to_string()),
            Token::Operator(Operator::Not),
            Token::Atom("b".to_string()),
            Token::Operator(Operator::And),
        ]
    );
}

// Test formula parsing
#[test]
fn test_formula_parsing() {
    let parser = FormulaParser::new("a & b | c");
    let formula = parser.parse();
    assert_eq!(
        formula.variables,
        ["a", "b", "c"].into_iter().map(String::from).collect()
    );

    // We can't directly compare the AST structure, so we'll test it indirectly
    // through evaluation
}

// Test formula evaluation
#[test]
fn test_formula_evaluation() {
    let parser = FormulaParser::new("a & b | c");
    let formula = parser.parse();

    let vars = [("a", true), ("b", false), ("c", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(true));

    let vars = [("a", true), ("b", false), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(false));
}

// Test operator precedence
#[test]
fn test_operator_precedence() {
    let parser = FormulaParser::new("a | b & c");
    let formula = parser.parse();

    let vars = [("a", false), ("b", true), ("c", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(true));

    let vars = [("a", false), ("b", true), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(false));
}

// Test complex formula with multiple operators
#[test]
fn test_complex_formula() {
    let parser = FormulaParser::new("(a | b) & ~c -> d <-> e");
    let formula = parser.parse();

    let vars = [
        ("a", true),
        ("b", false),
        ("c", false),
        ("d", true),
        ("e", true),
    ]
    .iter()
    .map(|&(s, b)| (s.to_string(), b))
    .collect();

    assert_eq!(formula.eval(&vars), Some(true));

    let vars = [
        ("a", false),
        ("b", false),
        ("c", true),
        ("d", false),
        ("e", true),
    ]
    .iter()
    .map(|&(s, b)| (s.to_string(), b))
    .collect();

    assert_eq!(formula.eval(&vars), Some(true));
}

// Test error handling for invalid input
#[test]
fn test_invalid_variable() {
    let parser = FormulaParser::new("a & b");
    let formula = parser.parse();

    let vars = [("a", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), None);
}

// Test associativity of operators
#[test]
fn test_operator_associativity() {
    let parser = FormulaParser::new("a -> b -> c");
    let formula = parser.parse();

    let vars = [("a", true), ("b", false), ("c", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(true));

    let vars = [("a", true), ("b", true), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), Some(false));
}
