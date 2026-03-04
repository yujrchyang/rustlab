pub fn run_slice() {
    test_slice();
    test_slice_str();
    test_slice_arr();
    test_slice_vec();
}

fn test_slice() {
    let mut arr = [1, 2, 3, 4, 5];
    let arr_slice1 = &arr[..=1];
    println!("{:?}", arr_slice1);

    let arr_slice2 = &mut arr[..=1];
    arr_slice2[0] = 111;
    println!("{:?}", arr_slice2);
    println!("{:?}", arr);
}

fn test_slice_str() {
    let s = String::from("hello world!");
    let s1 = &s[6..];
    let s2 = &s1;
    println!("{:?}", s1);
    println!("{:?}", s2);
}

fn test_slice_arr() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    println!("b: {:?}", b);
}

fn test_slice_vec() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    let b = &v[1..3];
    println!("b: {:?}", b);
}
