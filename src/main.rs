use truth_table_rs::{Formula, FormulaParser};

fn main() {
    // let source = "((A & B) | C)";
    let source: Vec<String> = std::env::args().skip(1).collect();
    let formulas: Vec<_> = source
        .iter()
        .map(|e| e.as_str())
        .map(FormulaParser::new)
        .map(FormulaParser::parse)
        .collect();

    // formula.print_truth_table();
    print_truth_table(&formulas);
}

fn print_truth_table(formulas: &[Formula]) {
    let mut all_variables = std::collections::HashSet::new();
    for formula in formulas {
        all_variables.extend(formula.variables.iter().cloned());
    }
    let mut variables: Vec<_> = all_variables.into_iter().collect();
    variables.sort_unstable();

    // Print header
    print!("| ");
    for var in &variables {
        print!("{} | ", var);
    }
    for formula in formulas.iter() {
        print!("{} | ", formula);
    }
    println!();

    // Print separator
    print!("|");
    for _ in &variables {
        print!(":-:|");
    }
    for _ in formulas {
        print!(":-:|");
    }
    println!();

    let num_vars = variables.len();
    let num_rows = 1 << num_vars;
    let mut vars = std::collections::HashMap::new();

    for i in (0..num_rows).rev() {
        print!("| ");
        for (j, var) in variables.iter().enumerate() {
            let value = (i >> (num_vars - 1 - j)) & 1 == 1;
            vars.insert(var.to_string(), value);
            let value_str = if value { "T" } else { "F" };
            print!("{} | ", value_str);
        }

        // Evaluate and print result for each formula
        for formula in formulas {
            let result_str = match formula.eval(&vars) {
                Some(true) => "T",
                Some(false) => "F",
                None => "E",
            };
            print!("{} | ", result_str);
        }
        println!();
    }

    println!("\nT: True, F: False, E: Error (undefined variable)");

    // Print expressions
}
