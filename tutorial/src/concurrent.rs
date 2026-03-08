pub fn run_concurrent() {
    test_thread_create();
    test_thread_move();
    test_mpsc();
    test_mutex();
    test_rwlock();
}

#[allow(unused_imports)]
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn test_rwlock() {
    let shared_data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut threads = vec![];

    for i in 0..5 {
        let data = Arc::clone(&shared_data);
        threads.push(thread::spawn(move || {
            let vec = data.read().unwrap();
            println!("Thread {} read data: {:?}", i, vec);
            thread::sleep(Duration::from_millis(1));
        }));
    }

    for i in 0..2 {
        let data = Arc::clone(&shared_data);
        threads.push(thread::spawn(move || {
            let mut vec = data.write().unwrap();
            println!("Thread {} write data", i);
            vec.push(i);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("shared data at the end: {:?}", shared_data);
}

fn test_mutex() {
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }
    // can not compile
    // {
    //     let counter = Mutex::new(0);
    //     let mut handles = vec![];
    //     for _ in 0..10 {
    //         thread::spawn(move || {
    //             let mut num = counter.lock().unwrap();
    //             *num += 1;
    //         });
    //         // counter already moved
    //         handles.push(counter);
    //     }
    // }
    // can not compile
    // {
    //     let counter = Rc::new(Mutex::new(0));
    //     let mut handles = vec![];
    //     for _ in 0..10 {
    //         let counter = Rc::clone(&counter);
    //         // Rc is not threads safely
    //         thread::spawn(move || {
    //             let mut num = counter.lock().unwrap();
    //             *num += 1;
    //         });
    //     }
    // }

    {
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
        println!("result: {}", *counter.lock().unwrap());
    }
}

fn test_mpsc() {
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // val ownership already got by tx.send()
            // println!("{:?}", val);
        });

        let rec = rx.recv().unwrap();
        println!("got: {}", rec);
    }

    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for item in vals {
                tx.send(item).unwrap();
            }
        });

        for r in rx {
            println!("Got: {}", r);
        }
    }

    {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for item in vals {
                tx.send(item).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("long"),
                String::from("time"),
                String::from("no"),
                String::from("see"),
            ];

            for item in vals {
                tx1.send(item).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        for r in rx {
            println!("Got: {}", r);
        }
    }
}

fn test_thread_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });
    handle.join().unwrap();
    // v already moved
    // println!("Here is a vector: {:?}", v);
}

fn test_thread_create() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spwaned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
