use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;

pub fn main(){
    async_comm();
    sync_comm();
}

fn sync_comm() {
    let bound = 3;
    let (tx,rx) = sync_channel(bound);

    for i in 0..3 {
        let tx = tx.clone();

        // cloned tx would be dropped
        thread::spawn(move || tx.send(i).unwrap());
    }

    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }
}

fn async_comm() {
    let (tx,rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        assert!(0 <= j && j < 10);
        println!("Channel Message Recieved : {}", j);
    }
}
