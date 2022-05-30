pub fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    _quick_sort(array, 0, array.len() - 1);
}

fn _quick_sort<T: PartialOrd>(array: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }

    let m = partition(array, lo, hi);
    _quick_sort(array, lo, m - 1);
    _quick_sort(array, m + 1, hi);
}

fn partition<T: PartialOrd>(array: &mut [T], lo: usize, hi: usize) -> usize {
    let mut l = lo;
    let mut r = hi + 1;

    loop {
        l += 1;
        while array[l] < array[lo] {
            l += 1;
            if l == hi {
                break;
            }
        }

        r -= 1;
        while array[r] > array[lo] {
            r -= 1;
            if r == lo {
                break;
            }
        }

        if l >= r {
            break;
        }
        array.swap(l, r);
    }

    array.swap(lo, r);

    r
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

        quick_sort(&mut random_vec);
        random_vec_copy.sort();

        assert_eq!(random_vec, random_vec_copy);
    }
}
