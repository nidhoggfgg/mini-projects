pub fn search<T: PartialOrd>(key: T, array: &[T]) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let middle = low + (high - low) / 2;
        if key < array[middle] {
            high = middle - 1;
        } else if key > array[middle] {
            low = middle + 1;
        } else {
            return Some(middle);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn search_with_isize() {
        let some: [isize; 7] = [-2, -1, 3, 6, 7, 8, 9];
        assert_eq!(2, search(3, &some).unwrap());
    }

    #[test]
    fn search_with_i64() {
        let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
        assert_eq!(8, search(10, &some).unwrap());
    }

    #[test]
    fn not_find() {
        let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
        assert_eq!(None, search(1, &some));
    }

    #[test]
    fn at_start() {
        let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
        assert_eq!(0, search(-10, &some).unwrap());
    }

    #[test]
    fn at_end() {
        let some: [i64; 10] = [-10, -9, -3, 3, 4, 5, 6, 9, 10, 11];
        assert_eq!(9, search(11, &some).unwrap());
    }
}
