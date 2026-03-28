use stdext::function_name;

pub fn run_variable() {
    test_var_func();
    test_var_return();
    test_struct();
}

fn test_struct() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };
    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, String::from("Hello world!")));
    println!(
        "{}: event1 - {:?}, event2 - {:?}, event3 - {:?}",
        function_name!(),
        event1,
        event2,
        event3
    );
}

#[derive(Debug)]
#[allow(dead_code)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct TopicId(u64);

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn test_var_return() {
    let is_pi = pi();
    let is_uint1 = not_pi();
    let is_uint2 = {
        pi();
    };
    println!(
        "{}: is_pi: {:?}, is_uint1: {:?}, is_uint2: {:?}",
        function_name!(),
        is_pi,
        is_uint1,
        is_uint2
    );
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn test_var_func() {
    println!(
        "{}: apply square {}",
        function_name!(),
        test_var_func_apply(2, test_var_func_square)
    );
    println!(
        "{}: apply cube {}",
        function_name!(),
        test_var_func_apply(2, test_var_func_cube)
    );
}

fn test_var_func_apply(val: i32, f: fn(i32) -> i32) -> i32 {
    f(val)
}

fn test_var_func_square(val: i32) -> i32 {
    val * val
}

fn test_var_func_cube(val: i32) -> i32 {
    val * val * val
}
