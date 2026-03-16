use std::cmp::Ordering;

// k is 1-based
pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if input.is_empty() || k == 0 || k > input.len() {
        return None;
    }

    let kth = _kth_smallest(input, k, 0, input.len() - 1);
    Some(kth)
}

fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
where
    T: PartialOrd + Copy,
{
    if lo == hi {
        return input[lo];
    }

    let pivot_idx = partition(input, lo, hi);
    let rank = pivot_idx - lo + 1;
    match k.cmp(&rank) {
        Ordering::Equal => input[pivot_idx],
        Ordering::Less => _kth_smallest(input, k, lo, pivot_idx - 1),
        Ordering::Greater => _kth_smallest(input, k - rank, pivot_idx + 1, hi),
    }
}

fn partition<T: PartialOrd>(input: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let mut left = lo;
    let mut right = hi;

    while left < right {
        while left < right && input[right] >= input[pivot] {
            right -= 1;
        }
        while left < right && input[left] <= input[pivot] {
            left += 1;
        }
        if left != right {
            input.swap(left, right);
        }
    }

    input.swap(pivot, left);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(&mut zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let mut one = [1];
        let first = kth_smallest(&mut one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(&mut many, 1);
        let third = kth_smallest(&mut many, 3);
        let sixth = kth_smallest(&mut many, 6);
        let fourteenth = kth_smallest(&mut many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
