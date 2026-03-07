pub fn run_iterator() {
    test_lazy();
    test_iter1();
    test_iter2();
    test_iter3();
    test_iter4();
}

fn test_lazy() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = arr
        .iter()
        .map(|x| {
            println!("calc: {} * 2", x);
            x * 2
        })
        .take(2);
    let r: Vec<i32> = iter.collect();
    println!("{:?}", r);
}

#[derive(Debug, Clone, Copy)]
struct Counter {
    cnt: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.cnt += 1;
        if self.cnt < 6 { Some(self.cnt) } else { None }
    }
}

fn test_iter1() {
    // modify source
    let mut c1 = Counter { cnt: 0 };
    for item in &mut c1 {
        println!("iter1: item = {}", item);
    }
    println!("iter1: c1 = {:?}", c1);

    // make a copy
    let c1 = Counter { cnt: 0 };
    for item in c1 {
        println!("iter1: item = {}", item);
    }
    println!("iter1: c1 = {:?}", c1);
}

fn test_iter2() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("v1_iter: {}", v);
    }
    // v1 can use again
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("v1_iter again: {}", v);
    }

    let mut v2 = vec![1, 2, 3];
    let v2_iter = v2.iter_mut();
    for v in v2_iter {
        println!("v2_iter before: {}", v);
        if *v > 1 {
            *v = 1
        }
        println!("v2_iter after: {}", v);
    }

    let v3 = vec![1, 2, 3];
    let v3_iter = v3.into_iter();
    for v in v3_iter {
        println!("v3_iter: {}", v);
    }
    // v3 can not use again
    // let v3_iter = v3.into_iter();
}

fn test_iter3() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("sum is {}", sum);

    let mut v2_iter = v1.iter();
    if let Some(_) = v2_iter.next() {
        println!("still has item");
    }
}

fn test_iter4() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
    let v3: Vec<_> = v1.iter().map(|x| x.to_string()).collect();
    println!("{:?}", v3);
}
