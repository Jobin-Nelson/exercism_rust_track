use std::collections::HashMap;

fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut note_map = HashMap::new();
    for word in note {
        note_map
            .entry(word)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for word in magazine {
        note_map
            .entry(word)
            .and_modify(|v| *v -= 1);
    }

    note_map
        .values()
        .all(|&x| x <= 0)
}

fn main() {
    let magazine = "two times three is not four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "two times two is four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));

    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}
