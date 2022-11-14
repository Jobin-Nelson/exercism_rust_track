pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

pub struct Position(i16, i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divmod_eq() {
        assert_eq!(divmod(10, 3), (3, 1));
        assert_eq!(divmod(25, 3), (8, 1));
    }
    
    #[test]
    fn manhattan_sum() {
        let pos = Position(10, 20);
        assert_eq!(pos.manhattan(), 30);
    }
}
