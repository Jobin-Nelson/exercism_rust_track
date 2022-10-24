fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

struct Position(i16, i16);

impl Position {
    fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}

fn main() {
    println!("divmod(10, 3) is {:?}", divmod(10, 3));

    let mut even_ints = evens(0_u8..);
    println!(
        "evens {}, {}, {}, {}",
        even_ints.next().unwrap(),
        even_ints.next().unwrap(),
        even_ints.next().unwrap(),
        even_ints.next().unwrap()
    );

    let mut evens_from_odds = evens(1_i16..);
    println!(
        "evens {}, {}, {}, {}",
        evens_from_odds.next().unwrap(),
        evens_from_odds.next().unwrap(),
        evens_from_odds.next().unwrap(),
        evens_from_odds.next().unwrap()
    );

    println!("manhattan of (3, 4) is {}", Position(3, 4).manhattan());
}
