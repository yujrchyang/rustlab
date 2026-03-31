use clap::Parser;

mod basic;

#[derive(Parser, Debug)]
#[command(author, version, about = "RustLab Tutorial Exercise")]
struct Args {
    #[arg(short, long, default_value = "hello")]
    exercise: String,
}

fn main() {
    let args = Args::parse();

    match args.exercise.as_str() {
        "hello" => run_hello(),
        "var" => basic::variable::run_variable(),
        "ctrl" => basic::control::run_control(),
        "composite" => basic::composite::run_composite(),
        _ => {
            eprintln!(
                "Error: Exercise '{}' not found. Please type --help to see available options.",
                args.exercise
            );
            std::process::exit(1)
        }
    }
}

fn run_hello() {
    println!("hello world!");
}
