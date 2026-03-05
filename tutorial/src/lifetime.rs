pub fn run_lifetime() {
    test_lifetime2();
    test_lifetime3();
    test_lifetime4();
}

// missing lifetime specifier
// fn test_lifetime1() {
//     fn test(x: &i32, y: &i32) -> &i32 {
//         if x > y { x } else { y }
//     }
// }

fn test_lifetime2() {
    fn bigger<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
        if x > y { x } else { y }
    }

    let x = 1;
    let y = 2;
    println!("bigger is {}", bigger(&x, &y));

    // z lifetime is short
    // let r;
    // {
    //     let z = 3;
    //     r = bigger(&x, &z);
    // }
    // println!("bigger is {}", r);
}

#[derive(Debug)]
#[allow(dead_code)]
struct A<'a> {
    name: &'a str,
}

fn test_lifetime3() {
    let s = "bob".to_string();
    let a = A { name: &s };
    println!("{:#?}", a);
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn test_lifetime4() {
    let x = "hello".to_string();
    let y = "world".to_string();
    let ann = 128;
    let r = longest_with_an_announcement(x.as_str(), y.as_str(), ann);
    println!("r = {}", r);
    println!("hello world!");
}
