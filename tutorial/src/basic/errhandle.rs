use stdext::function_name;
use thiserror::Error;

pub fn run_errhandle() {
    test_result();
}

fn test_result() {
    let r1 = test_result_division(1, 2);
    if let Ok(r) = r1 {
        println!("{}: devision ok {}", function_name!(), r);
    }

    let r2 = test_result_division(1, 0);
    if let Err(e) = r2 {
        println!("{}: devision err {:?}", function_name!(), e);
    }

    let r3 = test_result_ret();
    match r3 {
        Ok(r) => println!("{}: devision ok {}", function_name!(), r),
        Err(e) => println!("{}: devision err {:?}", function_name!(), e),
    }
}

fn test_result_ret() -> Result<i32, Errors> {
    let r1 = test_result_division(2, 2)?;
    println!("{}: devision ok {}", function_name!(), r1);
    let r2 = test_result_division(1, 0)?;
    println!("{}: devision ok {}", function_name!(), r2);
    Ok(0)
}

fn test_result_division(a: i32, b: i32) -> Result<i32, Errors> {
    if b == 0 {
        return Err(Errors::DivideByZero);
    }
    return Ok(a / b);
}

#[derive(Error, Debug)]
enum Errors {
    #[error("divide by zero")]
    DivideByZero,
}
