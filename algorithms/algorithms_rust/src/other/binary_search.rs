pub fn rank<T: PartialOrd>(key: T, array: &[T]) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = array.len() - 1;

    while low <= high {
        let middle: usize = low + (high - low) / 2;
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
