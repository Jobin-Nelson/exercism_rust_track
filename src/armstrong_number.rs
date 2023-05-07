pub fn is_armstrong_number(num: u32) -> bool {
    let mut stack = Vec::new();
    let mut mut_num = num;

    while mut_num != 0 {
        stack.push(mut_num % 10);
        mut_num = mut_num / 10;
    }

    let res: usize = stack.iter().fold(0, |mut acc, n| {
        acc += n.pow(stack.len() as u32) as usize;
        acc
    });

    res == num as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    fn test_there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    fn test_three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn test_four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }
    #[test]
    fn test_four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }
    #[test]
    fn test_ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999))
    }
    #[test]
    fn test_properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957))
    }
}
