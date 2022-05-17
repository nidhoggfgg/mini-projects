use algorithms::other::binary_search::rank;

#[test]
fn for_isize() {
    let some: [isize; 7] = [-2, -1, 3, 6, 7, 8, 9];
    assert_eq!(2, rank(3, &some).unwrap());
}

#[test]
fn for_i64() {
    let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
    assert_eq!(8, rank(10, &some).unwrap());
}

#[test]
fn not_find() {
    let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
    assert_eq!(None, rank(1, &some));
}

#[test]
fn at_start() {
    let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
    assert_eq!(0, rank(-10, &some).unwrap());
}

#[test]
fn at_end() {
    let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
    assert_eq!(9, rank(11, &some).unwrap());
}
