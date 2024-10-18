const SCORES: &[u64; 26] = &[
    1,  // 'A'
    3,  // 'B'
    3,  // 'C'
    2,  // 'D'
    1,  // 'E'
    4,  // 'F'
    2,  // 'G'
    4,  // 'H'
    1,  // 'I'
    8,  // 'J'
    5,  // 'K'
    1,  // 'L'
    3,  // 'M'
    1,  // 'N'
    1,  // 'O'
    3,  // 'P'
    10, // 'Q'
    1,  // 'R'
    1,  // 'S'
    1,  // 'T'
    1,  // 'U'
    4,  // 'V'
    4,  // 'W'
    8,  // 'X'
    4,  // 'Y'
    10, // 'Z'
];

pub fn score(word: &str) -> u64 {
    word.bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| SCORES[(c.to_ascii_lowercase() - b'a') as usize])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_letter() {
        let input = "a";
        let output = score(input);
        let expected = 1;
        assert_eq!(output, expected);
    }
    #[test]
    fn uppercase_letter() {
        let input = "A";
        let output = score(input);
        let expected = 1;
        assert_eq!(output, expected);
    }
    #[test]
    fn valuable_letter() {
        let input = "f";
        let output = score(input);
        let expected = 4;
        assert_eq!(output, expected);
    }
    #[test]
    fn short_word() {
        let input = "at";
        let output = score(input);
        let expected = 2;
        assert_eq!(output, expected);
    }
    #[test]
    fn short_valuable_word() {
        let input = "zoo";
        let output = score(input);
        let expected = 12;
        assert_eq!(output, expected);
    }
    #[test]
    fn medium_word() {
        let input = "street";
        let output = score(input);
        let expected = 6;
        assert_eq!(output, expected);
    }
    #[test]
    fn medium_valuable_word() {
        let input = "quirky";
        let output = score(input);
        let expected = 22;
        assert_eq!(output, expected);
    }
    #[test]
    fn long_mixed_case_word() {
        let input = "OxyphenButazone";
        let output = score(input);
        let expected = 41;
        assert_eq!(output, expected);
    }
    #[test]
    fn english_like_word() {
        let input = "pinata";
        let output = score(input);
        let expected = 8;
        assert_eq!(output, expected);
    }
    #[test]
    fn empty_input() {
        let input = "";
        let output = score(input);
        let expected = 0;
        assert_eq!(output, expected);
    }
    #[test]
    fn entire_alphabet_available() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = score(input);
        let expected = 87;
        assert_eq!(output, expected);
    }
    #[test]
    fn non_english_scrabble_letters_do_not_score() {
        let input = "piñata";
        let output = score(input);
        let expected = 7;
        assert_eq!(output, expected);
    }
    #[test]
    fn german_letters_do_not_score() {
        let input = "STRAßE";
        let output = score(input);
        let expected = 5;
        assert_eq!(output, expected);
    }
}
