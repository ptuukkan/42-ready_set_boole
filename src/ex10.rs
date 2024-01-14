fn morton(x: u32) -> u32 {
    println!("{:x}", x);
    x
} 

pub fn map(x: u16, y: u16) -> f64 {
    morton(x as u32);
    1.2
}
