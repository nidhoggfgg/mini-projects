pub fn merge_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    if array.len() > 1 {
        let mid = array.len() / 2;
        merge_sort(&mut array[..mid]);
        merge_sort(&mut array[mid..]);
        merge(array, mid);
    }
}

fn merge<T: PartialOrd + Copy>(array: &mut [T], mid: usize) {
    let aux = array.to_vec();
    let mut l = 0;
    let mut r = mid;

    for v in array {
        if r >= aux.len() || (l < mid && aux[l] < aux[r]) {
            *v = aux[l];
            l += 1;
        } else {
            *v = aux[r];
            r += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut array = Vec::<char>::new();
        merge_sort(&mut array);
        assert_eq!(array, vec![]);
    }

    #[test]
    fn one() {
        let mut array = vec![1];
        merge_sort(&mut array);
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn double() {
        let mut array = vec![4.5, 1.2, 3.4, 5.6];
        merge_sort(&mut array);
        assert_eq!(array, vec![1.2, 3.4, 4.5, 5.6]);
    }

    #[test]
    fn integer() {
        let mut array = vec![1, 4, 5, 9, 2, 6];
        merge_sort(&mut array);
        assert_eq!(array, vec![1, 2, 4, 5, 6, 9]);
    }

    #[test]
    fn range() {
        let mut array: Vec<u32> = (0..1000).rev().collect();
        merge_sort(&mut array);
        assert_eq!(array, (0..1000).collect::<Vec<u32>>());
    }
}
