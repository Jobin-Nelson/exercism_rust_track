pub fn square(s: u32) -> u64 {
    let err = "Square must be between 1 and 64";
    1_u64.checked_shl(s.checked_sub(1).expect(err)).expect(err)
}

pub fn total() -> u64 {
    u64::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_square_case(input: u32, expected: u64) {
        assert_eq!(square(input), expected);
    }

    #[test]
    fn test_1() {
        process_square_case(1, 1);
    }

    #[test]
    fn test_2() {
        process_square_case(2, 2);
    }

    #[test]
    fn test_3() {
        process_square_case(3, 4);
    }

    #[test]
    fn test_4() {
        process_square_case(4, 8);
    }

    #[test]
    fn test_16() {
        process_square_case(16, 32_768);
    }

    #[test]
    fn test_32() {
        process_square_case(32, 2_147_483_648);
    }

    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn test_square_0_raises_an_exception() {
        square(0);
    }

    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn test_square_greater_than_64_raises_an_exception() {
        square(65);
    }

    #[test]
    fn test_returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(total(), 18_446_744_073_709_551_615);
    }
}
