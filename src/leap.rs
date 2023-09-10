pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_leapyear_case(year: u64, expected: bool) {
        assert_eq!(is_leap_year(year), expected);
    }

    #[test]
    fn test_year_not_divisible_by_4_common_year() {
        process_leapyear_case(2015, false);
    }
    #[test]
    fn test_year_divisible_by_2_not_divisible_by_4_in_commmon_year() {
        process_leapyear_case(1970, false);
    }
    #[test]
    fn test_year_divisible_by_4_not_divisible_by_100_in_leap_year() {
        process_leapyear_case(1996, true);
    }
    #[test]
    fn test_year_divisible_by_4_and_5_is_still_a_leap_year() {
        process_leapyear_case(1960, true);
    }
    #[test]
    fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
        process_leapyear_case(2100, false);
    }
    #[test]
    fn test_year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
        process_leapyear_case(1900, false);
    }
    #[test]
    fn test_year_divisible_by_400_leap_year() {
        process_leapyear_case(2000, true);
    }
    #[test]
    fn test_year_divisible_by_200_not_divisible_by_400_common_year() {
        process_leapyear_case(1800, false);
    }
    #[test]
    fn test_early_years() {
        process_leapyear_case(1, false);
        process_leapyear_case(4, true);
        process_leapyear_case(100, false);
        process_leapyear_case(400, true);
        process_leapyear_case(900, false);
    }
}
