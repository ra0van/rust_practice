pub fn run_closure_ex() {
    println!("====== Clousres Example ========");

    // Closures
    let multiply_2 = |x | x * 2;

    println!("Multiply by 2: {}", multiply_2(5));

    // Borrowing
    let message = String::from("This is a message");

    let message_copy = message.clone();

    let printer = move || println!("{}", message);

    printer();

    println!("Message: {}", message_copy);

    println!("====== End Example ========");
}

