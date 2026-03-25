pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + Default,
{
    if arr.len() <= 1 {
        return;
    }
    return merge_sort_range(arr, 0, arr.len() - 1);
}

fn merge_sort_range<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: PartialOrd + Clone + Default,
{
    if lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        merge_sort_range(arr, lo, mid);
        merge_sort_range(arr, mid + 1, hi);
        merge_two_array(arr, lo, mid, hi);
    }
}

fn merge_two_array<T>(arr: &mut [T], lo: usize, mid: usize, hi: usize)
where
    T: PartialOrd + Clone + Default,
{
    let mut arr1 = arr[lo..=mid].to_vec();
    let mut arr2 = arr[mid + 1..=hi].to_vec();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            arr[lo + i + j] = std::mem::take(&mut arr1[i]);
            i += 1;
        } else {
            arr[lo + i + j] = std::mem::take(&mut arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        arr[lo + i + j] = std::mem::take(&mut arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        arr[lo + i + j] = std::mem::take(&mut arr2[j]);
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        merge_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut num_vec = vec![1, 9, 5, 7, 3];
        merge_sort(&mut num_vec);
        assert_eq!(num_vec, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_float_vec() {
        let mut num_vec = vec![1.1, 9.9, 5.5, 7.7, 3.3];
        merge_sort(&mut num_vec);
        assert_eq!(num_vec, vec![1.1, 3.3, 5.5, 7.7, 9.9]);
    }

    #[test]
    fn test_string_vec() {
        let mut str_vec = vec![
            String::from("alice"),
            String::from("david"),
            String::from("black"),
            String::from("bob"),
            String::from("carol"),
        ];
        merge_sort(&mut str_vec);
        assert_eq!(
            str_vec,
            vec![
                String::from("alice"),
                String::from("black"),
                String::from("bob"),
                String::from("carol"),
                String::from("david"),
            ]
        )
    }
}
