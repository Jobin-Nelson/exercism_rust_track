use core::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = array.len().checked_sub(1)?;

    while l <= r {
        let m = (l + r) >> 1;
        match key.cmp(array.get(m)?) {
            Ordering::Less => r = m.checked_sub(1)?,
            Ordering::Equal => return Some(m),
            Ordering::Greater => l = m + 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_a_value_in_an_array_with_one_element() {
        assert_eq!(find(&[6], 6), Some(0));
    }
    #[test]
    fn finds_first_value_in_an_array_with_two_element() {
        assert_eq!(find(&[1, 2], 1), Some(0));
    }
    #[test]
    fn finds_second_value_in_an_array_with_two_element() {
        assert_eq!(find(&[1, 2], 2), Some(1));
    }
    #[test]
    fn finds_a_value_in_the_middle_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }
    #[test]
    fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
    }
    #[test]
    fn nothing_is_included_in_an_empty_array() {
        assert_eq!(find(&[], 1), None);
    }
}
