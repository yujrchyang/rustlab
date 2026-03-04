pub fn run_generices() {
    test_struct();
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn test_struct() {
    let a = Point { x: 1, y: 2.0 };
    println!("{:#?}", a);

    let b = Point {
        x: true,
        y: String::from("hello"),
    };
    println!("{:#?}", b);

    let c = a.mixup(b);
    println!("{:#?}", c);
}
