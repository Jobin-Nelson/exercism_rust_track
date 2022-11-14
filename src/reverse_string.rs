use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) ->  String {
    input
        .graphemes(true)
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn reverse_test() {
        assert_eq!(reverse("hello"), "olleh".to_string());
        assert_eq!(reverse("jobin"), "niboj".to_string());
    }
}
