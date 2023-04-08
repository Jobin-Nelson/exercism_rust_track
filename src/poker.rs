pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    todo!();
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use super::*;

    fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
        let mut hs = HashSet::new();
        for item in input.iter() {
            hs.insert(*item);
        }
        hs
    }

    fn test(input: &[&str], expected: &[&str]) {
        assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
    }

    #[test]
    fn test_single_hand_always_wins() {
        test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
    }
    #[test]
    fn test_single_hand_always_wins() {
        test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
    }
    #[test]
    fn test_duplicate_hands_always_tie() {
        let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
        assert_eq!(&winning_hands(input), input)
    }
    #[test]
    fn test_highest_card_of_all_hands_wins() {
        test(
            &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
            &["3S 4S 5D 6H JH"],
        )
    }
    #[test]
    fn test_a_tie_has_multiple_winners() {
        test(
            &[
                "4D 5S 6S 8D 3C",
                "2S 4C 7S 9H 10H",
                "3S 4S 5D 6H JH",
                "3H 4H 5C 6C JD",
            ],
            &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
        )
    }
    #[test]
    fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
        // multiple hands with the same high cards, tie compares next highest ranked,
        // down to last card
        test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
    }
    #[test]
    fn test_one_pair_beats_high_card() {
        test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
    }
    #[test]
    fn test_highest_pair_wins() {
        test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
    }
    #[test]
    fn test_two_pairs_beats_one_pair() {
        test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
    }
    #[test]
    fn test_two_pair_ranks() {
        // both hands have two pairs, highest ranked pair wins
        test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
    }
    #[test]
    fn test_two_pairs_second_pair_cascade() {
        // both hands have two pairs, with the same highest ranked pair,
        // tie goes to low pair
        test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
    }
    #[test]
    fn test_two_pairs_last_card_cascade() {
        // both hands have two identically ranked pairs,
        // tie goes to remaining card (kicker)
        test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
    }
    #[test]
    fn test_three_of_a_kind_beats_two_pair() {
        test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
    }
    #[test]
    fn test_three_of_a_kind_ranks() {
        //both hands have three of a kind, tie goes to highest ranked triplet
        test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }
}
