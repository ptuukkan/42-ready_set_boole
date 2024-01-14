// https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
fn squish(mut n: u32) -> u16 {
    n &= 0x55555555; //                 n = -f-e -d-c -b-a -9-8 -7-6 -5-4 -3-2 -1-0
    n = (n ^ (n >> 1)) & 0x33333333; // n = --fe --dc --ba --98 --76 --54 --32 --10
    n = (n ^ (n >> 2)) & 0x0f0f0f0f; // n = ---- fedc ---- ba98 ---- 7654 ---- 3210
    n = (n ^ (n >> 4)) & 0x00ff00ff; // n = ---- ---- fedc ba98 ---- ---- 7654 3210
    n = (n ^ (n >> 8)) & 0x0000ffff; // n = ---- ---- ---- ---- fedc ba98 7654 3210
    return n as u16;
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    if n < 0.0 || n > 1.0 {
        println!("Error");
        return (0, 0);
    }

    let morton = n * u32::MAX as f64;
    let x = squish(morton as u32);
    let y = squish(morton as u32 >> 1);
    (x, y)
}

#[test]
fn test_reverse_map() {

}
