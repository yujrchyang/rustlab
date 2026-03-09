pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Clone + Default,
{
    let size = arr.len();
    if size <= 1 {
        return;
    }

    merge_sort_range(arr, 0, size - 1);
}

fn merge_sort_range<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: Ord + Clone + Default,
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
    T: Ord + Clone + Default,
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
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        merge_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
