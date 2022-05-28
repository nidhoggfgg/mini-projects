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
    use rand::prelude::*;

    #[test]
    fn random() {
        let mut rng = thread_rng();
        let mut random_vec: Vec<u32> = (1..100).collect();
        random_vec.shuffle(&mut rng);
        let mut random_vec_copy = random_vec.clone();

        merge_sort(&mut random_vec);
        random_vec_copy.sort();

        assert_eq!(random_vec, random_vec_copy);
    }
}
