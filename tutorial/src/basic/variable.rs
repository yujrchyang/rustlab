use stdext::function_name;

pub fn run_variable() {
    test_var_fun();
    test_var_ret();
}

fn test_var_ret() {
    let is_pi = test_var_ret_pi();
    let is_uint1 = test_var_ret_not_pi();
    let is_uint2 = {
        test_var_ret_pi();
    };
    println!(
        "{}: is_pi: {:?}, is_uint1: {:?}, is_uint2: {:?}",
        function_name!(),
        is_pi,
        is_uint1,
        is_uint2
    );
}

fn test_var_ret_pi() -> f64 {
    3.1415926
}

fn test_var_ret_not_pi() {
    3.1415926;
}

fn test_var_fun() {
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
