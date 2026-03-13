pub fn burrows_wheeler_transform(input: String) -> (String, usize) {
    let len = input.len();
    if len == 0 {
        return (String::new(), 0);
    }

    let chars: Vec<char> = input.chars().collect();
    let mut table = Vec::<(usize, String)>::with_capacity(len);
    for i in 0..len {
        let mut row = String::with_capacity(len);
        for j in 0..len {
            row.push(chars[(i + j) % len]);
        }
        table.push((i, row));
    }
    table.sort_by(|a, b| a.1.cmp(&b.1));

    let mut encode = String::with_capacity(len);
    let mut primary_index = 0;
    for (i, item) in table.iter().enumerate() {
        encode.push(item.1.chars().last().unwrap());
        if item.0 == 0 {
            primary_index = i;
        }
    }

    (encode, primary_index)
}

pub fn inv_burrows_wheeler_transform(input: (String, usize)) -> String {
    let len = input.0.len();
    if len == 0 {
        return String::new();
    }

    let chars: Vec<char> = input.0.chars().collect();
    let mut table: Vec<(usize, char)> = Vec::with_capacity(len);
    for i in 0..len {
        table.push((i, chars[i]));
    }
    table.sort_by(|a, b| a.1.cmp(&b.1));

    let mut decode = String::with_capacity(len);
    let mut primary_index = input.1;
    for _ in 0..len {
        decode.push(table[primary_index].1);
        primary_index = table[primary_index].0
    }

    decode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT".to_string())),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO".to_string())),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST".to_string())),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS".to_string())),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST".to_string())),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::".to_string())),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform(
                "!{}{}(((&&%%!??.=::".to_string()
            )),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]".to_string())),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("".to_string())),
            ""
        );
    }
}
