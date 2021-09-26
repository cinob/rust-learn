use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     // val 所有权已经转移
    //     // println!("val is {}", val);
    // });

    // // 阻塞主线程 直到有值传入通道
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hi2"),
            String::from("hi3"),
            String::from("hi4"),
            String::from("hi5"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("wahaha"),
            String::from("wahaha2"),
            String::from("wahaha3"),
            String::from("wahaha4"),
            String::from("wahaha5"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
