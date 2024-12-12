use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple produces single consumer <=> a channel can have multiple sending ends that produce values but only one receiving end to consume them
use std::sync::{Arc, Mutex};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // if we put the join() here, the main thread will stop and wait
    // for the spawned thread to finish. Being careful where we put the join() is really important.

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // when the main thread ends here, all spawned threads automatically shut down.
    // In this case, the spawned thread won't finish printing everything.

    // join() on a thread stops the other thread for working, in this case stops it from finishing
    // and the spawned thread saved in the variable handle will finish.
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("The v vector is: {:?}", v);
    });

    // drop(v); // this is just an example to see that the v borrowed in another thread might outlive
    // the v in the main thread.
    
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel(); // tuple (transmitter, receiver)\

    thread::spawn(move || {
        let val = String::from("hello from spawned thread to main thread");
        // we could instead use val here, before moving it
        tx.send(val).unwrap();
        // println!("{}", val); // we can't use val here because send() takes ownership over it. When the recv() receives it, the ownership will move to it.
    });

    let received = rx.recv().unwrap(); // this blocks the main thread and waits until rx has a value to receive
    // There is also .try_recv() which doesn't block. We can call it every now and then in a loop in case the thread has other work to do while waiting
    println!("Got: {}", received);

    // multiple transmitters, one receiver
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // we can clone the transmitter to use it multiple times
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // Mutex in a single threaded application
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // Sharing a Mutex between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
