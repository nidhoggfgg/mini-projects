pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();

    for i in 0..len {
        let mut min = i;
        for j in i..len {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(min, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut array = Vec::<char>::new();
        selection_sort(&mut array);
        assert_eq!(array, vec![]);
    }

    #[test]
    fn one() {
        let mut array = vec![1];
        selection_sort(&mut array);
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn double() {
        let mut array = vec![4.5, 1.2, 3.4, 5.6];
        selection_sort(&mut array);
        assert_eq!(array, vec![1.2, 3.4, 4.5, 5.6]);
    }

    #[test]
    fn integer() {
        let mut array = vec![1, 4, 5, 9, 2, 6];
        selection_sort(&mut array);
        assert_eq!(array, vec![1, 2, 4, 5, 6, 9]);
    }

    #[test]
    fn range() {
        let mut array: Vec<u32> = (0..1000).rev().collect();
        selection_sort(&mut array);
        assert_eq!(array, (0..1000).collect::<Vec<u32>>());
    }
}
