pub fn run_smartpointer() {
    test_box();
    test_deref();
    test_drop();
    test_rc();
    test_refcell();
    test_cycle_ref();
}

fn test_cycle_ref() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum List {
        Con(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Con(_, item) => Some(item),
                List::Nil => None,
            }
        }
    }

    let a = Rc::new(List::Con(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Con(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // create cycle link
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    //println!("a next item = {:?}", a.tail());
}

fn test_refcell() {
    let a = RefCell::new(1);
    {
        println!("before refcell a is {:?}", a);
        let mut v = a.borrow_mut();
        // check borrow at runtime, it will panic
        // let mut u = a.borrow_mut();
        // let u = a.borrow_mut();
        // let u = a.borrow();
        *v = 2;
    }
    println!("after refcell a is {:?}", a);
}

fn test_rc() {
    #[allow(dead_code)]
    enum List {
        Con(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(List::Con(5, Rc::new(List::Con(10, Rc::new(List::Nil)))));
    println!("counter after creating a is {}", Rc::strong_count(&a));
    let _b = List::Con(3, Rc::clone(&a));
    println!("counter after creating b is {}", Rc::strong_count(&a));
    {
        let _c = List::Con(4, Rc::clone(&a));
        println!("counter after creating c is {}", Rc::strong_count(&a));
    }
    println!("counter after release c is {}", Rc::strong_count(&a));
}

use std::cell::Ref;
use std::cell::RefCell;
use std::ops::Drop;
struct MyString(String);
impl Drop for MyString {
    fn drop(&mut self) {
        println!("string({}) is drop", self.0);
    }
}

fn test_drop() {
    {
        let _ms1 = MyString("hello".to_string());
        {
            let _ms2 = MyString("world".to_string());
        }
    }

    {
        let _ms3 = MyString("hello".to_string());
        let _ms4 = MyString("world".to_string());
        drop(_ms3);
        println!("after drop");
    }
}

use std::ops::Deref;
use std::rc::Rc;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_deref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // y is a pointer
    // assert_eq!(5, y);

    let b = Box::new(1);
    // b is a smart pointer box
    // assert_eq!(1, b);
    println!("b is {:?}", b);
    println!("value of b is {:?}", *b);

    let my = MyBox::new(666);
    println!("my is {}", *my);
}

trait Speak {
    fn speak(&self);
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("dog is speak");
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("cat is speak");
    }
}

fn test_box() {
    let b = Box::new(5);
    println!("{:?}", b);

    fn box_speak(a: Box<dyn Speak>) {
        a.speak();
    }
    let dog = Box::new(Dog);
    let cat = Box::new(Cat);
    box_speak(dog);
    box_speak(cat);

    let mut arr: Vec<Box<dyn Speak>> = Vec::new();
    arr.push(Box::new(Dog));
    arr.push(Box::new(Cat));
    for a in arr.iter() {
        a.speak();
    }
}
