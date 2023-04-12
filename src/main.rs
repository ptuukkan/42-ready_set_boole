use std::env;

mod ex00;
mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod helpers;

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
        _ => {
            println!("Unsupported argument");
        }
    };
}
