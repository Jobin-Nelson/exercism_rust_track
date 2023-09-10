pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) >> 1).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_of_sum_1() {
        assert_eq!(1, square_of_sum(1))
    }

    #[test]
    fn test_square_of_sum_5() {
        assert_eq!(225, square_of_sum(5))
    }

    #[test]
    fn test_sum_of_squares_1() {
        assert_eq!(1, sum_of_squares(1))
    }

    #[test]
    fn test_sum_of_squares_5() {
        assert_eq!(55, sum_of_squares(5))
    }

    #[test]
    fn test_difference_1() {
        assert_eq!(0, difference(1))
    }

    #[test]
    fn test_difference_5() {
        assert_eq!(170, difference(5))
    }
}
