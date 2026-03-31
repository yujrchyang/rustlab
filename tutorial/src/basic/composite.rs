use stdext::function_name;

pub fn run_composite() {
    test_struct();
    test_if_let();
}

#[allow(irrefutable_let_patterns)]
fn test_if_let() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    if let User { id, name, gender } = alice {
        println!("{}: get {:?}-{:?}-{:?}", function_name!(), id, name, gender)
    }
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
