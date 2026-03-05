use clap::Parser;

mod compositetype;
mod control;
mod function;
mod generics;
mod lifetime;
mod mtrait;
mod reference;
mod slice;

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
        "func" => function::run_function(),
        "ctrl" => control::run_control(),
        "ref" => reference::run_reference(),
        "slice" => slice::run_slice(),
        "ct" => compositetype::run_compositetype(),
        "gen" => generics::run_generices(),
        "trait" => mtrait::run_mtrait(),
        "lifetime" => lifetime::run_lifetime(),
        // You can add more module matches here later.
        // "ownership" => ownership::run_exercise(),
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
    println!("Hello, Rust world!");
}
