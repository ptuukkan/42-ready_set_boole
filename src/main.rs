use std::env;

mod dnf;
mod ex00;
mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod ex05;
mod ex06;
mod ex07;
mod ex08;
mod ex09;
mod helpers;
mod proposition;
mod propositional_formula;
mod set;

fn run_ex00(a: u32, b: u32) {
    println!("Adder: {} + {} = {}", a, b, ex00::adder(a, b));
}

fn run_ex01(a: u32, b: u32) {
    println!("Multiplier: {} * {} = {}", a, b, ex01::multiplier(a, b));
}

fn run_ex02(n: u32) {
    println!("Gray_code: {} = {}", n, ex02::gray_code(n));
}

fn run_ex03(formula: &str) {
    println!(
        "Eval_formula: {} = {}",
        formula,
        ex03::eval_formula(formula)
    );
}
fn run_ex04(formula: &str) {
    println!("Print_truth_table: {}", formula);
    ex04::print_truth_table(formula);
}
fn run_ex05(formula: &str) {
    println!(
        "Negation_normal_norm: {} = {}",
        formula,
        ex05::negation_normal_form(formula)
    );
}

fn run_ex06(formula: &str) {
    println!(
        "Conjunctive_normal_norm: {} = {}",
        formula,
        ex06::conjunctive_normal_form(formula)
    );
}

fn run_ex07(formula: &str) {
    println!("Sat: {} = {}", formula, ex07::sat(formula));
}

fn run_dnf(formula: &str) {
    println!(
        "Sat: {} = {}",
        formula,
        dnf::disjunctive_normal_form(formula)
    );
}

fn run_powerset(set: Vec<i32>) {
    dbg!(ex08::powerset(set));
}

fn run_eval_set(formula: &str, sets: Vec<Vec<i32>>) {
    dbg!(ex09::eval_set(formula, sets));
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "adder" => {
            let a = args[1].parse::<u32>().unwrap();
            let b = args[2].parse::<u32>().unwrap();
            run_ex00(a, b);
        }
        "multiplier" => {
            let a = args[1].parse::<u32>().unwrap();
            let b = args[2].parse::<u32>().unwrap();
            run_ex01(a, b);
        }
        "gray_code" => {
            let n = args[1].parse::<u32>().unwrap();
            run_ex02(n);
        }
        "eval_formula" => {
            run_ex03(args[1].as_str());
        }
        "print_truth_table" => {
            run_ex04(args[1].as_str());
        }
        "negation_normal_form" => {
            run_ex05(args[1].as_str());
        }
        "conjunctive_normal_form" => {
            run_ex06(args[1].as_str());
        }
        "sat" => {
            run_ex07(args[1].as_str());
        }
        "disjunctive_normal_form" => {
            run_dnf(args[1].as_str());
        }
        "powerset" => {
            run_powerset(
                args[1]
                    .as_str()
                    .split(" ")
                    .into_iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
        "eval_set" => {
            run_eval_set(
                args[1].as_str(),
                args[2]
                    .as_str()
                    .split(";")
                    .into_iter()
                    .map(|x| {
                        x.trim()
                            .split(" ")
                            .into_iter()
                            .map(|y| y.parse::<i32>().unwrap())
                            .collect()
                    })
                    .collect(),
            );
        }

        _ => {
            println!("Unsupported argument");
        }
    };
}
