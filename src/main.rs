mod ex00;
mod ex01;
mod ex02;
mod ex03;

fn main() {
    println!("{}", ex00::adder(5, 5));
	println!("{}", ex01::multiplier(1, 0));
	println!("{}", ex02::gray_code(8));
	println!("{}", ex03::eval_formula("a10|"));
}
