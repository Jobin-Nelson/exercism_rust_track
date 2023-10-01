pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    list.windows(2)
        .map(|a| format!("For want of a {} the {} was lost.\n", a[0], a[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list.first().unwrap()
        )))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_pieces() {
        let input = vec!["nail", "shoe"];
        let expected = [
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn three_pieces() {
        let input = vec!["nail", "shoe", "horse"];
        let expected = [
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn one_piece() {
        let input = vec!["nail"];
        let expected = String::from("And all for the want of a nail.");
        assert_eq!(build_proverb(&input), expected);
    }
}
