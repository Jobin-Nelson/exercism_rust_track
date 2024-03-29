#[macro_export]
macro_rules! hashmap {
    ( ) => { ::std::collections::HashMap::new() };
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut h = ::std::collections::HashMap::new();
        $(
            h.insert($k, $v);
        )*
        h
    }}
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_empty() {
        let expected: HashMap<u32, u32> = HashMap::new();
        let computed: HashMap<u32, u32> = hashmap!();
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_single() {
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        assert_eq!(hashmap!(1 => "one"), expected);
    }

    #[test]
    fn test_no_trailing_comma() {
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        expected.insert(2, "two");
        assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
    }

    #[test]
    fn test_trailing_comma() {
        let mut expected = HashMap::new();
        expected.insert('h', 89);
        expected.insert('a', 1);
        expected.insert('s', 19);
        expected.insert('h', 8);
        assert_eq!(
            hashmap!(
                'h' => 89,
                'a' => 1,
                's' => 19,
                'h' => 8,
            ),
            expected
        );
    }

    #[test]
    fn test_nested() {
        let mut expected = HashMap::new();
        expected.insert("non-empty", {
            let mut subhashmap = HashMap::new();
            subhashmap.insert(23, 623);
            subhashmap.insert(34, 21);
            subhashmap
        });
        expected.insert("empty", HashMap::new());
        assert_eq!(
            hashmap!(
                "non-empty" => hashmap!(
                    23 => 623,
                    34 => 21
                ),
                "empty" => hashmap!()
            ),
            expected
        );
    }
}
