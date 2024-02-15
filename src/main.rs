use std::sync::{Arc, Mutex};

mod struct_trait;
use crate::struct_trait::Animal;

mod generics;
mod closures;
mod mpsc_ex;

#[path = "./assignment/task.rs"]
mod task;
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

fn run_trait_ex(){
    println!("====== Run Trait Ex======");
    let dog = struct_trait::Dog {
        name: String::from("Rusty"),
    };
    println!("{}", dog.make_sound());
    print_twice(dog.name);
    println!("====== End Trait Ex======");
}


fn execute() {
    sum_of_array();
    run_trait_ex();
    generics::run_generic_ex();
    closures::run_closure_ex();
    mpsc_ex::main();

    task::main();
}

fn main() {
    execute();
}
