use truth_table_rs::Formula;

fn main() {
    // let source = "((A & B) | C)";
    let source = std::env::args().nth(1).unwrap_or("".to_string());
    let formula: Formula = (source.as_str()).into();

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
