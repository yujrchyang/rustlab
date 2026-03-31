use stdext::function_name;

pub fn run_control() {
    test_if();
    test_loop();
    test_while();
    test_for();
}

fn test_for() {
    for i in 0..10 {
        println!("{}: i is {}", function_name!(), i);
    }
}

fn test_while() {
    let mut i = 0;
    while i <= 10 {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("{}: {} is odd-number", function_name!(), i);
    }
}

fn test_loop() {
    let mut i = 0;
    loop {
        if i % 2 == 0 {
            println!("{}: {} is even", function_name!(), i);
        }

        i += 1;
        if i > 10 {
            break;
        }
    }
}

fn test_if() {
    let n = 5;
    if n > 10 {
        println!("{}: n > 10", function_name!());
    } else if n > 5 {
        println!("{}: n > 5", function_name!());
    } else {
        println!("{}: n <= 5", function_name!())
    }
}
