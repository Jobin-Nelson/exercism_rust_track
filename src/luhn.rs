pub fn is_valid(code: &str) -> bool {
    let mut mul = 1;
    let mut sum = 0;
    let mut size = 0;

    for c in code.chars().rev() {
        if c.is_ascii_whitespace() {
            continue;
        }
        sum += match c.to_digit(10) {
            Some(n) => {
                let res = n * mul;
                if res > 9 {
                    res - 9
                } else {
                    res
                }
            },
            None => return false,
        };

        mul ^= 3;
        size += 1;
    }

    sum % 10 == 0 && size > 1
}

#[cfg(test)]
mod test {
    use super::*;

    fn process_valid_case(number: &str, is_luhn_expected: bool) {
        assert_eq!(is_valid(number), is_luhn_expected);
    }

    #[test]
    fn test_single_digit_strings_can_not_be_valid() {
            process_valid_case("1", false);
    }
    
    #[test]
    fn test_a_single_zero_is_invalid() {
        process_valid_case("0", false);
    }

    #[test]
    fn test_a_valid_canadian_sin() {
        process_valid_case("055 444 285", true);
    }

    #[test]
    fn test_invalid_canadian_sin() {
        process_valid_case("055 444 286", false);
    }

    #[test]
    fn test_valid_number_with_an_even_number_of_digits() {
        process_valid_case("095 245 88", true);
    }
}

