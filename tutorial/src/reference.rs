pub fn run_reference() {
    test_overhanging_reference();
    test_mutable();
}

fn test_overhanging_reference() {
    fn dangle() -> String {
        let s = String::from("hello");
        s
    }
    let s = dangle();
    println!("s is {s}");

    // can not compile
    // fn dangle2() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
}

fn test_mutable() {
    let a = 1;
    let b = 2;
    let mut s = "hello".to_string();

    let x = &a;
    println!("x is {}", x);
    // can't assign twice
    // x = &b;
    // println!("x is {}", x);

    let mut y = &b;
    println!("y is {}", y);
    // can assign twice because y is mutable
    y = &a;
    println!("y is {}", y);

    let z = &s;
    println!("z is {}", z);
    // z is immutable, so can't chang s
    // z.push_str(" world");
    // println!("z is {}", z);

    let x = &mut s;
    println!("x is {}", x);
    x.push_str(" world");
    println!("x is {}", x);
    println!("s is {}", s);
}
