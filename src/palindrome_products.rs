#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        is_palindrome(value).then_some(Palindrome(value))
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(value: u64) -> bool {
    if value % 10 == 0 {
        return false;
    }

    let mut reverse = 0;
    let mut r = value;
    while r > 0 {
        reverse = reverse * 10 + r % 10;
        r /= 10;
    }
    value == reverse
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p = None;
    let mut max_p = None;

    for i in min..=max {
        for j in i..=max {
            let value = i * j;
            if is_palindrome(value) {
                if value < min_p.unwrap_or(u64::MAX) {
                    min_p = Some(value);
                }
                if value > max_p.unwrap_or(u64::MIN) {
                    max_p = Some(value);
                }
            }
        }
    }

    match (min_p, max_p) {
        (Some(min), Some(max)) => {
            Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test `Palindrome::new` with valid input
    fn palindrome_new_return_some() {
        for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
            assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
        }
    }
    #[test]
    /// test `Palindrome::new` with invalid input
    fn palindrome_new_return_none() {
        for v in [12, 2322, 23443, 1233211, 8932343] {
            assert_eq!(Palindrome::new(v), None);
        }
    }
    #[test]
    fn find_the_smallest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9).map(|(min, _)| min.into_inner());
        let expected = Some(1);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_largest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9).map(|(_, max)| max.into_inner());
        let expected = Some(9);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_smallest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99).map(|(min, _)| min.into_inner());
        let expected = Some(121);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_largest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99).map(|(_, max)| max.into_inner());
        let expected = Some(9009);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_smallest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999).map(|(min, _)| min.into_inner());
        let expected = Some(10201);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_largest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999).map(|(_, max)| max.into_inner());
        let expected = Some(906609);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_smallest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999).map(|(min, _)| min.into_inner());
        let expected = Some(1002001);
        assert_eq!(output, expected);
    }
    #[test]
    fn find_the_largest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999).map(|(_, max)| max.into_inner());
        let expected = Some(99000099);
        assert_eq!(output, expected);
    }
    #[test]
    fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(1002, 1003).map(|(min, _)| min.into_inner());
        let expected = None;
        assert_eq!(output, expected);
    }
    #[test]
    fn empty_result_for_largest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(15, 15).map(|(_, max)| max.into_inner());
        let expected = None;
        assert_eq!(output, expected);
    }
    #[test]
    fn error_result_for_smallest_if_min_is_more_than_max() {
        let output = palindrome_products(10000, 1).map(|(min, _)| min.into_inner());
        let expected = None;
        assert_eq!(output, expected);
    }
    #[test]
    fn error_result_for_largest_if_min_is_more_than_max() {
        let output = palindrome_products(2, 1).map(|(_, max)| max.into_inner());
        let expected = None;
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_product_does_not_use_the_smallest_factor() {
        let output = palindrome_products(3215, 4000).map(|(min, _)| min.into_inner());
        let expected = Some(10988901);
        assert_eq!(output, expected);
    }
}
