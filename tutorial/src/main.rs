use clap::Parser;

mod closure;
mod collection;
mod compositetype;
mod concurrent;
mod control;
mod errhandle;
mod function;
mod generics;
mod iterator;
mod lifetime;
mod mtrait;
mod reference;
mod slice;
mod smartpointer;

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
        "err" => errhandle::run_errhandle(),
        "closure" => closure::run_closure(),
        "iter" => iterator::run_iterator(),
        "coll" => collection::run_collection(),
        "spointer" => smartpointer::run_smartpointer(),
        "concurrent" => concurrent::run_concurrent(),
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
