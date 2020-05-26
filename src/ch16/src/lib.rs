use std::thread;
use std::time::Duration;

pub fn printch15_1() {
    println!("---- ここからch15-1 ---");
    let handle = thread::spawn ( || {
        for i in 1..10 {
            println!("number {} from the spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn printlnch16_5() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn (move || {
        println!("Here's a vector: {:?}", v);
        // これは move なしで動くが？
        // 暗黙的にmoveされてるんやな
        // for i in v {
        //     println!("number {} from the spawn thread", i);
        //     thread::sleep(Duration::from_millis(1));
        // }
    });


    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

use std::sync::mpsc;

fn ch16_11() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec!(
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        );
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // clone したやつのdropも待ってくれるのか
    });

    // let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec!(
            String::from("more 2"),
            String::from("mesages 2"),
            String::from("for 2"),
            String::from("you 2"),
        );
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        // unwrap しなくていいのん？
        println!("Got: {}", received);
    }
}