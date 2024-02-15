use std::sync::{Arc, Mutex};

mod struct_trait;
use crate::struct_trait::Animal;

// Sum of array in parallel
fn sum_of_array(){
    let mut arr = vec![];

    for i in 0..1000 {
        arr.push(i);
    }

    let sum = Arc::new(Mutex::new(0));


    let threads: Vec<_> = arr.into_iter()
        .map(|i| {
            let sum = Arc::clone(&sum);
            std::thread::spawn(move || {
                let mut sum = sum.lock().unwrap();
                *sum += i;
            })
        }).collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("sum: {}", *sum.lock().unwrap());
}

fn print_twice<T> (value: T) where T: std::fmt::Debug {
    println!("{:#?} {:#?}", value, value);
    
}

struct Point<T> {
    x: T,
    y: T,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    sum_of_array();
    println!("Hello World");

    let dog = struct_trait::Dog {
        name: String::from("Rusty"),
    };
    println!("{}", dog.make_sound());
    print_twice(dog.name);

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
    
    // Closures
    let multiply_2 = |x | x * 2;

    println!("Multiply by 2: {}", multiply_2(5));

    // Borrowing
    let message = String::from("This is a message");

    let message_copy = message.clone();

    let printer = move || println!("{}", message);

    printer();

    println!("Message: {}", message_copy);
}

