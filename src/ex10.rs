// https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
fn spread(mut n: u32) -> u32 {
    n = (n ^ (n << 8)) & 0x00ff00ff; // n = ---- ---- fedc ba98 ---- ---- 7654 3210
    n = (n ^ (n << 4)) & 0x0f0f0f0f; // n = ---- fedc ---- ba98 ---- 7654 ---- 3210
    n = (n ^ (n << 2)) & 0x33333333; // n = --fe --dc --ba --98 --76 --54 --32 --10
    n = (n ^ (n << 1)) & 0x55555555; // n = -f-e -d-c -b-a -9-8 -7-6 -5-4 -3-2 -1-0
    n
}

pub fn map(x: u16, y: u16) -> f64 {
    let m_x = spread(x as u32);
    let m_y = spread(y as u32) << 1;
    let morton = f64::from(m_x + m_y);
    morton / u32::MAX as f64
}
