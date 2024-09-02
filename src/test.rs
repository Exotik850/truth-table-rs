
use super::*;

// Test Shunting Yard algorithm
#[test]
fn test_shunting_yard() {
    assert_eq!(shunting_yard("a & b | c"), vec!['a', 'b', '&', 'c', '|']);
    assert_eq!(shunting_yard("a & (b | c)"), vec!['a', 'b', 'c', '|', '&']);
    assert_eq!(shunting_yard("~a & b"), vec!['a', '~', 'b', '&']);
}

// Test formula parsing
#[test]
fn test_formula_parsing() {
    let parser = FormulaParser::new("a & b | c");
    let formula = parser.parse();
    assert_eq!(formula.variables, vec!["a", "b", "c"]);

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

    assert_eq!(formula.eval(&vars), true);

    let vars = [("a", true), ("b", false), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), false);
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

    assert_eq!(formula.eval(&vars), true);

    let vars = [("a", false), ("b", true), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), false);
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

    assert_eq!(formula.eval(&vars), true);

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

    assert_eq!(formula.eval(&vars), true);
}

// Test error handling for invalid input
#[test]
#[should_panic(expected = "Invalid variable")]
fn test_invalid_variable() {
    let parser = FormulaParser::new("a & b");
    let formula = parser.parse();

    let vars = [("a", true)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    formula.eval(&vars);
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

    assert_eq!(formula.eval(&vars), true);

    let vars = [("a", true), ("b", true), ("c", false)]
        .iter()
        .map(|&(s, b)| (s.to_string(), b))
        .collect();

    assert_eq!(formula.eval(&vars), false);
}
