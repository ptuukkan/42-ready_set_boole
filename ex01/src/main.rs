fn adder(a: u32, b: u32) -> u32 {
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;
    let mut temp_sum: u32;

    while carry != 0 {
        temp_sum = sum ^ carry;
        carry = (sum & carry) << 1;
        sum = temp_sum;
    }
    sum
}

fn main() {
    println!("{}", adder(2000, 8000));
}

#[test]
fn test_adder() {
    for i in 0..100 {
        for j in 0..100 {
            assert_eq!(adder(i, j), i + j);
        }
    }
}
