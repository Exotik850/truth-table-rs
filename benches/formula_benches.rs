use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use truth_table_rs::{FormulaParser};

fn parse_benchmark(c: &mut Criterion) {
    let source = "a & b | ~c -> d <-> e";

    c.bench_function("parse_formula", |b| {
        b.iter(|| {
            let parser = FormulaParser::new(source);
            parser.parse()
        })
    });
}

fn eval_benchmark(c: &mut Criterion) {
    let source = "a & b | ~c -> d <-> e";
    let parser = FormulaParser::new(source);
    let formula = parser.parse();

    let vars = [
        ("a", true),
        ("b", false),
        ("c", true),
        ("d", false),
        ("e", true),
    ]
    .iter()
    .map(|&(s, b)| (s.to_string(), b))
    .collect::<HashMap<_, _>>();

    c.bench_function("eval_formula", |b| b.iter(|| formula.eval(&vars)));
}

fn print_truth_table_benchmark(c: &mut Criterion) {
    let source = "a & b | ~c -> d <-> e";
    let parser = FormulaParser::new(source);
    let formula = parser.parse();

    c.bench_function("print_truth_table", |b| {
        b.iter(|| formula.print_truth_table())
    });
}

criterion_group!(
    benches,
    parse_benchmark,
    eval_benchmark,
    print_truth_table_benchmark
);
criterion_main!(benches);
