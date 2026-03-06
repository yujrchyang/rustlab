pub fn run_closure() {
    test_closure1();
    test_closure2();
    test_closure3();
    test_closure4();
    test_closure5();
}

fn test_closure1() {
    let use_closure = || {
        println!("this is a closure");
    };

    use_closure();
}

fn test_closure2() {
    let get_x = |x| x;
    let n = get_x(5);
    println!("get_x return {}", n);
    // closure can only with one type
    // let _ = get_x(5.0);
}

fn call_once(c: impl FnOnce()) {
    c();
}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_fn(c: impl Fn()) {
    c()
}

fn test_closure3() {
    let s1 = "hello".to_string();
    let clourse_once = move || {
        println!("clourse_once: {}", s1);
    };
    call_once(clourse_once);
    // ownership already moved
    // call_once(clourse_once);

    let mut s2 = "hello".to_string();
    let mut clourse_mut = || {
        s2.push_str(", world");
        println!("clourse_mut: {}", s2);
    };
    call_mut(&mut clourse_mut);
    call_mut(&mut clourse_mut);
    call_once(clourse_mut);
    println!("out of clourse: {}", s2);

    let s3 = "hello".to_string();
    let mut clourse_fn = || {
        println!("clourse_fn: {}", s3);
    };
    call_fn(clourse_fn);
    call_fn(clourse_fn);
    call_mut(&mut clourse_fn);
    call_once(clourse_fn);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_closure4() {
    println!("do_twice return: {}", do_twice(add_one, 1));
}

fn return_clourse() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn test_closure5() {
    let c = return_clourse();
    println!("return_clourse: {}", c(1));
}
