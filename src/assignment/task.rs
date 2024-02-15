use std::{
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
struct Task {
    id: u32,
    payload: String,
}

fn create_task(id: u32, payload: String) -> Task {
    Task {id: id, payload: payload}
}

struct Worker {
    id: u32,
}

impl Worker{
    fn process_task(&self, task: Task) -> String {
        let rsp = format!(
            "Worker {} has processed task {} with payload {}",
            self.id, task.id, task.payload);
        rsp.to_string()
    }

}

fn create_worker(id: u32) -> Worker {
    Worker {id: id}
}

pub fn main(){
    let (sender, receiver) = channel::<Task>();
    let receiver = Arc::new(Mutex::new(receiver));

    let (result_tx, result_rx) = channel::<String>();

    let tasks = Vec::from_iter(
        (1..=10).map(|i| create_task(i, format!("Task {}", i).to_string())),
    );

    let workers = Vec::from_iter(
        (1..=5).map(|i| create_worker(i)),
    );

    let mut threads = vec![];

    for worker in workers {
        let thread_receiver = Arc::clone(&receiver);
        let thread_result_tx = result_tx.clone();

        let handle = thread::spawn(move || {
            let mut counter = 0;
            while let Ok(task) = thread_receiver.lock().unwrap().recv() {
                let result = worker.process_task(task);
                thread_result_tx.send(result).unwrap();
                counter += 1;
                if counter == 2 {
                    break;
                }
            }
        });

        threads.push(handle);
    }

    for task in tasks{
        let thread_tx = sender.clone();
        thread_tx.send(task).expect("Failed to send a task");
    }

    drop(sender);
    drop(result_tx);

    loop {
        match result_rx.recv_timeout(Duration::from_secs(10)) {
            Ok(result) => println!("Result => {}", result),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("No more results within 10 seconds");
                break;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("Results channel disconnected");
                break;
            }
        }
    }

    for handle in threads {
        handle.join().expect("Failed to join thread");    
    }
}

