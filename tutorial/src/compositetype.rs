pub fn run_compositetype() {
    test_color();
    test_dummy();
    test_person();
    test_enmu();
    test_if_let();
}

struct Color(i32, i32, i32);

impl Color {
    fn print(&self) {
        println!("Color: ({}, {}, {})", self.0, self.1, self.2);
    }
}

fn test_color() {
    let red = Color(255, 0, 0);
    red.print();
}

struct Dummy;

impl Dummy {
    fn do_something(&self) {
        println!("Doing dummy things ...");
    }
}

fn test_dummy() {
    let d = Dummy;
    d.do_something();
}

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn greet(&self) {
        println!(
            "Hi, my name is {} and I'm {} years old.",
            self.name, self.age
        );
    }
}

fn test_person() {
    let p = Person {
        name: String::from("alice"),
        age: 30,
    };
    p.greet();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move to ({}, {})", x, y),
            Message::Write(text) => println!("write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

fn test_enmu() {
    let msg = Message::Write(String::from("hello"));
    msg.process();
    let m2 = Message::ChangeColor(255, 0, 0);
    m2.process();
    let m3 = Message::Quit;
    m3.process();
    let m4 = Message::Move { x: 0, y: 0 };
    m4.process();
}

fn test_if_let() {
    fn devide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot devide zero"))
        } else {
            Ok(a / b)
        }
    }

    let r1 = devide(4.0, 2.0);
    if let Ok(val) = r1 {
        println!("result is {}", val);
    }

    let r2 = devide(4.0, 0.0);
    if let Err(err) = r2 {
        println!("Error is {}", err);
    }
}
