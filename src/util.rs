pub fn leading_zeros(num: u8) -> usize {
    for i in 0..8 {
        let msb = 1 << (7 - i);
        if (num & msb) != 0 {
            return i;
        }
    }

    8
}

#[test]
fn test_leading_zeros() {
    let mut i = 0;
    let mut bit_leading_zeros = 8;
    while bit_leading_zeros != 0 {
        assert_eq!(leading_zeros(i), bit_leading_zeros);
        if i == 0 || i == 1 {
            i += 1
        } else {
            i *= 2
        }
        bit_leading_zeros -= 1;
    }
    assert_eq!(leading_zeros(i), bit_leading_zeros);
}
