pub fn run_function() {
    test_return_value();
}

fn test_return_value() {
    let a = 1u32;
    let b = a + 1;

    let c = {
        let d = 2u32;
        b + d
    };
    println!("test func: c is {c}");
}
