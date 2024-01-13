use std::collections::{BTreeMap, HashSet};

use crate::propositional_formula::PropositionalFormula;

impl PropositionalFormula {
    pub fn print_truth_table(&self) {
        let mask: u32 = 1 << self.variables.len();

        print_header(&self.variables);

        for m in 0..mask {
            let variable_map = self.get_variable_map(m);
            let result = self.evaluate(&variable_map);
            print_row(&variable_map, result);
        }
    }

    pub fn get_variable_map(&self, mask: u32) -> BTreeMap<char, bool> {
        let mut variable_map: BTreeMap<char, bool> = BTreeMap::new();
        let mut vars = Vec::from_iter(&self.variables);
        vars.sort();

        for (i, c) in vars.iter().rev().enumerate() {
            variable_map.insert(*c.to_owned(), (mask >> i & 1) == 1);
        }
        variable_map
    }
}

fn print_header(variables: &HashSet<char>) {
    let mut vars = Vec::from_iter(variables);
    vars.sort();

    let header = vars
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" | ");

    println!("| {} | = |", header);

    let border = vars
        .iter()
        .map(|_v| "-".to_string())
        .collect::<Vec<String>>()
        .join("-|-");

    println!("|-{}-|---|", border);
}

fn print_row(variable_map: &BTreeMap<char, bool>, result: bool) {
    let row = variable_map
        .iter()
        .map(|v| match *v.1 {
            true => "1",
            false => "0",
        })
        .collect::<Vec<&str>>()
        .join(" | ");

    println!(
        "| {} | {} |",
        row,
        match result {
            true => "1",
            false => "0",
        }
    );
}

pub fn print_truth_table(formula: &str) {
    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        pf.print_truth_table();
    } else {
        println!("Error");
    }
}
