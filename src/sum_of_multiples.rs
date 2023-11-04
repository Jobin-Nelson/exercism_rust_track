pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.iter().any(|&f| f > 0 && n % f == 0))
        .sum()

    // let mut hs = HashSet::new();
    // for m in factors {
    //     let total_factor = (limit - 1) / m;
    //     for f in 0..=total_factor {
    //         hs.insert(f * m);
    //     }
    // }
    // hs.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(78, sum_of_multiples(20, &[3, 5]));
    }
    #[test]
    fn no_multiples_within_limit() {
        assert_eq!(0, sum_of_multiples(1, &[3, 5]));
    }
    #[test]
    fn one_factor_has_multiples_within_limit() {
        assert_eq!(3, sum_of_multiples(4, &[3, 5]));
    }
    #[test]
    fn more_than_one_multiple_within_limit() {
        assert_eq!(9, sum_of_multiples(7, &[3]));
    }
    #[test]
    fn more_than_one_factor_with_multiples_within_limit() {
        assert_eq!(23, sum_of_multiples(10, &[3, 5]));
    }
    #[test]
    fn as_much_larger_limit() {
        assert_eq!(233_168, sum_of_multiples(1000, &[3, 5]));
    }
    #[test]
    fn three_factors() {
        assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]));
    }
    #[test]
    fn some_pairs_of_factors_relatively_prime_and_some_not() {
        assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]));
    }
    #[test]
    fn all_numbers_are_multiples_of_1() {
        assert_eq!(4950, sum_of_multiples(100, &[1]));
    }
}
