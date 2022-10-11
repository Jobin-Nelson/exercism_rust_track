#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty() || first_list
        .windows(second_list.len())
        .any(|x| x == second_list);

    let sublist = first_list.is_empty() || second_list
        .windows(first_list.len())
        .any(|x| x == first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

fn main() {
    let (a1, b1) = ([1, 2, 3], [1, 2, 3, 4, 5]);
    let (a2, b2) = ([3, 4, 5], [1, 2, 3, 4, 5]);
    let (a3, b3) = ([3, 4], [1, 2, 3, 4, 5]);
    let (a4, b4) = ([1, 2, 3], [1, 2, 3]);
    let (a5, b5) = ([1, 2, 3, 4, 5], [2, 3, 4]);
    let (a6, b6) = ([1, 2, 4], [1, 2, 3, 4, 5]);

    println!("{:?}", sublist(&a1, &b1));
    println!("{:?}", sublist(&a2, &b2));
    println!("{:?}", sublist(&a3, &b3));
    println!("{:?}", sublist(&a4, &b4));
    println!("{:?}", sublist(&a5, &b5));
    println!("{:?}", sublist(&a6, &b6));
}
