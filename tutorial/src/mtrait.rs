pub fn run_mtrait() {
    test_trait();
    test_trait_bound();
    test_trait_obj();
}

trait GetInfo {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32 {
        0u32
    }
}

fn print_info(item: &impl GetInfo) {
    println!("print_info: name is {}", item.get_age());
    println!("print_info: age is {}", item.get_age());
}

struct Student {
    name: String,
    age: u32,
}

impl GetInfo for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

struct Teacher {
    name: String,
    age: u32,
}

impl GetInfo for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn test_trait() {
    let s = Student {
        name: "bob".to_string(),
        age: 18,
    };
    println!("student info is ({}, {})", s.get_name(), s.get_age());
    print_info(&s);

    let t = Teacher {
        name: "Black".to_string(),
        age: 40,
    };
    println!("teacher info is ({}, {})", t.get_name(), t.get_age());
    print_info(&t);
}

trait GetName {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

impl<T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("print_name {}", self.get_name());
    }
}

trait GetAge {
    fn get_age(&self) -> u32;
}

struct PeopleMatchInfo<T, U> {
    master: T,
    employee: U,
}

impl<T, U> PeopleMatchInfo<T, U>
where
    T: GetName + GetAge,
    U: GetName + GetAge,
{
    fn print_all_info(&self) {
        println!("teacher name = {}", self.master.get_name());
        println!("teacher age = {}", self.master.get_age());
        println!("student name = {}", self.employee.get_name());
        println!("student age = {}", self.employee.get_age());
    }
}

struct TTeacher {
    name: String,
    age: u32,
}

impl GetName for TTeacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for TTeacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

struct SStudent {
    name: String,
    age: u32,
}

impl GetName for SStudent {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for SStudent {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn test_trait_bound() {
    let t = TTeacher {
        name: "TTeacher".to_string(),
        age: 50,
    };
    let s = SStudent {
        name: "SStudent".to_string(),
        age: 16,
    };
    let m = PeopleMatchInfo {
        master: t,
        employee: s,
    };
    m.print_all_info();

    let ss = SStudent {
        name: "SSStudent".to_string(),
        age: 16,
    };
    ss.print_name();
}

struct SchoolMember<'a>(&'a dyn GetName);

impl<'a> SchoolMember<'a> {
    fn print_name(&self) {
        println!("schoolmember: name is {}", self.0.get_name());
    }
}

fn test_trait_obj() {
    let alice = SStudent {
        name: "alice".to_string(),
        age: 10,
    };
    let bob = TTeacher {
        name: "bob".to_string(),
        age: 60,
    };

    let sm1 = SchoolMember(&alice);
    sm1.print_name();
    let sm2 = SchoolMember(&bob);
    sm2.print_name();
}
