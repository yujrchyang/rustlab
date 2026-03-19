pub fn insertion_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    if arr.len() <= 1 {
        return;
    }
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn test_float_vec() {
        let mut num_vec = vec![1.1, 9.9, 5.5, 7.7, 3.3];
        insertion_sort(&mut num_vec);
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
