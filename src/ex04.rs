use std::collections::BTreeMap;

use crate::helpers::*;

fn print_header(value_map: &BTreeMap<char, bool>) {
    let header = value_map
        .iter()
        .map(|v| v.0.to_string())
        .collect::<Vec<String>>()
        .join(" | ");

    println!("| {} | = |", header);

    let border = value_map
        .iter()
        .map(|_v| "-".to_string())
        .collect::<Vec<String>>()
        .join("-|-");

    println!("|-{}-|---|", border);
}

fn print_row(value_map: &BTreeMap<char, bool>, result: bool) {
    let row = value_map
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
    if let Ok(parse_result) = parse_formula(formula) {
        let high_mask: u32 = 1 << parse_result.operands.len();
        
        print_header(&parse_result.operands);

        for mask in 0..high_mask {
            let value_map = get_value_map(&parse_result.operands, &mask);
            let result = evaluate(&parse_result.tree, &value_map);
            print_row(&value_map, result);
        }
    } else {
        println!("Error");
    }
}
