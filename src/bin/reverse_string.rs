use unicode_segmentation::UnicodeSegmentation;

fn reverse(input: &str) ->  String {
    input
        .graphemes(true)
        .rev()
        .collect::<String>()
}

fn main() {
    assert_eq!(reverse("hello"), "olleh".to_string());
    println!("{} -> {}","hello".to_string(), reverse("hello"));
    assert_eq!(reverse("jobin"), "niboj".to_string());
    println!("{} -> {}","jobin".to_string(), reverse("jobin"));
}
