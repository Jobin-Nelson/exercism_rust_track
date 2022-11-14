#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|x| x == second_list);

    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|x| x == first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sublist_test() {
        let (a1, b1) = ([1, 2, 3], [1, 2, 3, 4, 5]);
        let (a2, b2) = ([3, 4, 5], [1, 2, 3, 4, 5]);
        let (a3, b3) = ([3, 4], [1, 2, 3, 4, 5]);

        assert_eq!(sublist(&a1, &b1), Comparison::Sublist);
        assert_eq!(sublist(&a2, &b2), Comparison::Sublist);
        assert_eq!(sublist(&a3, &b3), Comparison::Sublist);
    }

    #[test]
    fn equal_test() {
        let (a4, b4) = ([1, 2, 3], [1, 2, 3]);
        assert_eq!(sublist(&a4, &b4), Comparison::Equal);
    }

    #[test]
    fn superlist_test() {
        let (a5, b5) = ([1, 2, 3, 4, 5], [2, 3, 4]);
        assert_eq!(sublist(&a5, &b5), Comparison::Superlist);
    }

    #[test]
    fn unequal_test() {
        let (a6, b6) = ([1, 2, 4], [1, 2, 3, 4, 5]);
        assert_eq!(sublist(&a6, &b6), Comparison::Unequal);
    }
}
