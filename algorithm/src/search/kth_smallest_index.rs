use std::cmp::Ordering;

// k is 1-based
pub fn kth_smallest_index<T: PartialOrd>(input: &[T], k: usize) -> Option<usize> {
    if input.is_empty() || k == 0 || k > input.len() {
        return None;
    }

    let mut indices: Vec<usize> = (0..input.len()).collect();
    let idx = _kth_smallest_index(input, &mut indices, k - 1, 0, input.len() - 1);
    Some(indices[idx])
}

fn _kth_smallest_index<T: PartialOrd>(
    input: &[T],
    indices: &mut [usize],
    k: usize,
    lo: usize,
    hi: usize,
) -> usize {
    if lo == hi {
        return lo;
    }

    let idx = partition(input, indices, lo, hi);
    match k.cmp(&idx) {
        Ordering::Equal => return idx,
        Ordering::Less => _kth_smallest_index(input, indices, k, lo, idx - 1),
        Ordering::Greater => _kth_smallest_index(input, indices, k, idx + 1, hi),
    }
}

fn partition<T: PartialOrd>(input: &[T], indices: &mut [usize], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let mut left = lo;
    let mut right = hi;

    while left < right {
        while left < right && input[indices[right]] >= input[indices[pivot]] {
            right -= 1;
        }
        while left < right && input[indices[left]] <= input[indices[pivot]] {
            left += 1;
        }

        if left != right {
            indices.swap(left, right);
        }
    }

    indices.swap(pivot, left);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let zero: [u8; 0] = [];
        let rev = kth_smallest_index(&zero, 1);
        assert_eq!(None, rev);
    }

    #[test]
    fn test_zero() {
        let one = vec![1];
        let rev = kth_smallest_index(&one, 0);
        assert_eq!(None, rev);
    }

    #[test]
    fn test_beyond() {
        let one = vec![1];
        let rev = kth_smallest_index(&one, 10);
        assert_eq!(None, rev);
    }

    #[test]
    fn test_one_element() {
        let one = vec![1];
        let rev = kth_smallest_index(&one, 1);
        assert_eq!(0, rev.unwrap());
    }

    #[test]
    fn test_many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest_index(&many, 1);
        let third = kth_smallest_index(&many, 3);
        let sixth = kth_smallest_index(&many, 6);
        let eight = kth_smallest_index(&many, 8);
        let night = kth_smallest_index(&many, 9);
        let fourteenth = kth_smallest_index(&many, 14);

        assert_eq!(0, many[first.unwrap()]);
        assert_eq!(3, many[third.unwrap()]);
        assert_eq!(7, many[sixth.unwrap()]);
        assert_eq!(9, many[eight.unwrap()]);
        assert_eq!(9, many[night.unwrap()]);
        assert_eq!(17, many[fourteenth.unwrap()]);
    }
}
