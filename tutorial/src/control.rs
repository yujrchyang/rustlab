pub fn run_control() {
    test_if();
    test_let_if();
    test_loop();
    test_while();
    test_for();
}

fn test_if() {
    let a = 2;
    if a > 5 {
        println!("a > 5");
    } else if a > 4 {
        println!("a > 4");
    } else {
        println!("a <= 4");
    }
}

fn test_let_if() {
    let a = 3;
    let a_bigger_than_two: bool = if a > 2 { true } else { false };
    if a_bigger_than_two {
        println!("a > 2")
    } else {
        println!("a <= 2")
    }
}

fn test_loop() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 10 {
            break;
        }
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    }
}

fn test_while() {
    let mut cnt = 0;
    while cnt < 10 {
        println!("cnt = {:?}", cnt);
        cnt += 1;
    }
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];
    for item in a {
        println!("the value is: {:?}", item)
    }
}
