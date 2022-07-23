pub fn gray_code(n: u32) -> u32 {
	n ^ (n >> 1)
}

#[test]
fn test_gray_code() {
    assert_eq!(gray_code(0), 0);
	assert_eq!(gray_code(1), 1);
	assert_eq!(gray_code(2), 3);
	assert_eq!(gray_code(3), 2);
	assert_eq!(gray_code(4), 6);
	assert_eq!(gray_code(9), 13);
	assert_eq!(gray_code(12), 10);
	assert_eq!(gray_code(53920), 48112);
}
