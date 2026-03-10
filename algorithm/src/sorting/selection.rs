pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();
    if size <= 1 {
        return;
    }

    for i in 0..size - 1 {
        let mut min_idx = i;
        for j in i + 1..size {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if i != min_idx {
            arr.swap(i, min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        selection_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        selection_sort(&mut vec);
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
        selection_sort(&mut vec);
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
