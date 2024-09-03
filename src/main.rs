use truth_table_rs::FormulaParser;

fn main() {
    let source = "a | ~a";
    let parser = FormulaParser::new(source);
    let formula = parser.parse();

    // for node in formula.root.children() {
    //     println!("{:?}", node);
    // }

    // println!("{:?}", formula);
    formula.print_truth_table();

    // let vars = [("a", true), ("b", false), ("c", true)]
    //     .iter()
    //     .map(|&(s, b)| (s.to_string(), b))
    //     .collect();

    // println!("{}", formula.eval(&vars));
}
