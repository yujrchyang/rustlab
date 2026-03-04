pub fn run_reference() {
    test_overhanging_reference();
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
