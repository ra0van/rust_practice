enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn run_generic_ex(){
    println!("====== Generics Example ========");
    // Generics
    let pass: Result<i32, &str> = Result::Ok(5);

    let fail: Result<i32, &str> = Result::Err("Something went wrong");

    match pass {
        Result::Ok(value) => println!("Success: {}", value), 
        Result::Err(value) => println!("Failure: {}", value),
    }

    match fail{
        Result::Ok(value) => println!("Success: {}", value), 
        Result::Err(value) => println!("Failure: {}", value),
    }
    println!("====== End Generics Example ======");
}

