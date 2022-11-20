use std::collections::{HashMap, HashSet};

pub fn get_counter(word: &str) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for c in word.chars() {
        counter.entry(c).and_modify(|v| *v += 1).or_insert(0);
    }
    counter
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut result = HashSet::new();
    let expected_counts = get_counter(&word);
    for w in possible_anagrams {
        if w.to_lowercase() != word && get_counter(w) == expected_counts {
            result.insert(*w);
        }
    }
    result
    // possible_anagrams
    //     .iter()
    //     .filter(|w| w.to_lowercase() != word && get_counter(&w.to_lowercase()) == expected_counts)
    //     .copied()
    //     .collect()
}

#[allow(dead_code)]
fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_matches() {
        let word = "diaper";
        let inputs = ["hello", "world", "zombies", "pants"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn test_eliminate_anagram_subsets() {
        let word = "good";
        let inputs = ["dog", "goody"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn test_multiple_anagrams() {
        let word = "allergy";
        let inputs = [
            "gallery",
            "ballerina",
            "regally",
            "clergy",
            "largely",
            "leading",
        ];
        let outputs = vec!["gallery", "regally", "largely"];
        process_anagram_case(word, &inputs, &outputs);
    }
}
