pub fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 1..array.len() {
        for j in (1..i).rev() {
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
    fn normal() {
        // this is not good, just for fun :)
        let mut array = vec![4.5, 1.2, 3.4, 5.6];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1.2, 3.4, 4.5, 5.6]);
    }
}
