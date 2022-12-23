use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    if input.is_empty() {
        return res;
    }

    let input = input.join("");
    if input.is_empty() {
        return res;
    }

    let mut churn = input.chars();
    let real_worker_count = worker_count.min(input.len());
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut work_length = (input.len() / real_worker_count).max(1);
    if work_length * real_worker_count < input.len() {
        work_length += 1;
    }

    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        let my_thread = thread::spawn(move || {
            let mut map = HashMap::<char, usize>::new();
            for c in chunk.chars() {
                if c.is_alphabetic() {
                    map.entry(c.to_ascii_lowercase())
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
            map
        });
        thread_pool.push(my_thread);
    }

    for my_thread in thread_pool {
        let map = my_thread.join().unwrap();
        for (key, val) in map.iter() {
            res.entry(*key).and_modify(|v| *v += val).or_insert(*val);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    // Poem by Friedrich Schiller. The corresponding music is the European Anthem.
    const ODE_AN_DIE_FREUDE: [&str; 8] = [
        "Freude schöner Götterfunken",
        "Tochter aus Elysium,",
        "Wir betreten feuertrunken,",
        "Himmlische, dein Heiligtum!",
        "Deine Zauber binden wieder",
        "Was die Mode streng geteilt;",
        "Alle Menschen werden Brüder,",
        "Wo dein sanfter Flügel weilt.",
    ];
    // Dutch national anthem
    const WILHELMUS: [&str; 8] = [
        "Wilhelmus van Nassouwe",
        "ben ik, van Duitsen bloed,",
        "den vaderland getrouwe",
        "blijf ik tot in den dood.",
        "Een Prinse van Oranje",
        "ben ik, vrij, onverveerd,",
        "den Koning van Hispanje",
        "heb ik altijd geëerd.",
    ];
    // American national anthem
    const STAR_SPANGLED_BANNER: [&str; 8] = [
        "O say can you see by the dawn's early light,",
        "What so proudly we hailed at the twilight's last gleaming,",
        "Whose broad stripes and bright stars through the perilous fight,",
        "O'er the ramparts we watched, were so gallantly streaming?",
        "And the rockets' red glare, the bombs bursting in air,",
        "Gave proof through the night that our flag was still there;",
        "O say does that star-spangled banner yet wave,",
        "O'er the land of the free and the home of the brave?",
    ];

    #[test]
    fn test_no_texts() {
        assert_eq!(frequency(&[], 4), HashMap::new());
    }

    #[test]
    fn test_one_letter() {
        let mut hm = HashMap::new();
        hm.insert('a', 1);
        assert_eq!(frequency(&["a"], 4), hm);
    }

    #[test]
    fn test_case_insensitivity() {
        let mut hm = HashMap::new();
        hm.insert('a', 2);
        assert_eq!(frequency(&["aA"], 4), hm);
    }

    #[test]
    fn test_punctuation_doesnt_count() {
        assert!(!frequency(&WILHELMUS, 4).contains_key(&','));
    }

    #[test]
    fn test_all_three_anthems_1_worker() {
        let mut v = Vec::new();
        for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
            for line in anthem.iter() {
                v.push(*line);
            }
        }
        let freqs = frequency(&v[..], 1);
        assert_eq!(freqs.get(&'a'), Some(&49));
        assert_eq!(freqs.get(&'t'), Some(&56));
        assert_eq!(freqs.get(&'ü'), Some(&2));
    }
}
