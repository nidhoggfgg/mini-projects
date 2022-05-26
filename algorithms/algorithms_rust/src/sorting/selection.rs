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
    fn normal() {
        // this is not good, just for fun :)
        let mut array = vec![4.5, 1.2, 3.4, 5.6];
        selection_sort(&mut array);
        assert_eq!(array, vec![1.2, 3.4, 4.5, 5.6]);
    }
}
