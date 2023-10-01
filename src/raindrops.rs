pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    if n % 3 == 0 {
        res.push_str("Pling")
    }
    if n % 5 == 0 {
        res.push_str("Plang")
    }
    if n % 7 == 0 {
        res.push_str("Plong")
    }
    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("1", raindrops(1));
    }
    #[test]
    fn test_3() {
        assert_eq!("Pling", raindrops(3));
    }
    #[test]
    fn test_5() {
        assert_eq!("Plang", raindrops(5));
    }
    #[test]
    fn test_7() {
        assert_eq!("Plong", raindrops(7));
    }
    #[test]
    fn test_6() {
        assert_eq!("Pling", raindrops(6));
    }
    #[test]
    fn test_8() {
        assert_eq!("8", raindrops(8));
    }
    #[test]
    fn test_10() {
        assert_eq!("Plang", raindrops(10));
    }
    #[test]
    fn test_15() {
        assert_eq!("PlingPlang", raindrops(15));
    }
    #[test]
    fn test_3125() {
        assert_eq!("Plang", raindrops(3125));
    }
    #[test]
    fn test_12121() {
        assert_eq!("12121", raindrops(12121));
    }
}
