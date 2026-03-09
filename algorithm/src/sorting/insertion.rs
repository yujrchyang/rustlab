pub fn insertion_sort<T: Ord>(arr: &mut Vec<T>) {
    let size = arr.len();
    if size <= 1 {
        return;
    }

    for i in 1..size {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_binary_search<T: Ord>(arr: &mut [T]) {
    let size = arr.len();
    if size <= 1 {
        return;
    }

    for i in 1..size {
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|pos| pos);
        let mut j = i;
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod insertion_sort {
        use super::*;

        #[test]
        fn test_empty_vec() {
            let mut empty_vec: Vec<String> = vec![];
            insertion_sort(&mut empty_vec);
            assert_eq!(empty_vec, Vec::<String>::new());
        }

        #[test]
        fn test_number_vec() {
            let mut num_vec = vec![1, 9, 5, 7, 3];
            insertion_sort(&mut num_vec);
            assert_eq!(num_vec, vec![1, 3, 5, 7, 9]);
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
            insertion_sort(&mut str_vec);
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

    mod insertion_sort_binary_search {
        use super::*;

        #[test]
        fn test_empty_vec() {
            let mut empty_vec: Vec<String> = vec![];
            insertion_sort_binary_search(&mut empty_vec);
            assert_eq!(empty_vec, Vec::<String>::new());
        }

        #[test]
        fn test_number_vec() {
            let mut num_vec = vec![1, 9, 5, 7, 3];
            insertion_sort_binary_search(&mut num_vec);
            assert_eq!(num_vec, vec![1, 3, 5, 7, 9]);
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
            insertion_sort_binary_search(&mut str_vec);
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
}
