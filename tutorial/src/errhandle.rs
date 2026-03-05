pub fn run_errhandle() {
    test_errhandle1();
    test_unwrap();
    test_transmit();
}

fn produce_error(switch: bool) -> Result<u32, &'static str> {
    if switch { Err("produced error") } else { Ok(1) }
}

fn transmit_error(flag: bool) -> Result<String, &'static str> {
    let _ = produce_error(flag)?;
    Ok("transmit ok".to_string())
}

fn test_errhandle1() {
    let r = produce_error(true);
    match r {
        Ok(_) => {
            println!("there is no error");
        }
        Err(e) => {
            println!("errior is: {}", e);
        }
    }
}

fn test_unwrap() {
    // this will panic
    // let r1 = produce_error(true).unwrap();
    // let _ = produce_error(true).expect("hello world");
    let r = produce_error(false).unwrap();
    println!("result is {}", r);
}

fn test_transmit() {
    let r1 = transmit_error(true);
    if let Err(e) = r1 {
        println!("error result is {}", e);
    }

    let r2 = transmit_error(false).unwrap();
    println!("normal result is {}", r2);
}
