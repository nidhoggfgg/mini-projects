pub fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 1..array.len() {
        for j in (1..=i).rev() {
            if array[j] > array[j - 1] {
                break;
            }
            array.swap(j, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut array = Vec::<char>::new();
        insertion_sort(&mut array);
        assert_eq!(array, vec![]);
    }

    #[test]
    fn one() {
        let mut array = vec![1];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn double() {
        let mut array = vec![4.5, 1.2, 3.4, 5.6];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1.2, 3.4, 4.5, 5.6]);
    }

    #[test]
    fn integer() {
        let mut array = vec![1, 4, 5, 9, 2, 6];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 4, 5, 6, 9]);
    }

    #[test]
    fn range() {
        let mut array: Vec<u32> = (0..1000).rev().collect();
        insertion_sort(&mut array);
        assert_eq!(array, (0..1000).collect::<Vec<u32>>());
    }
}
