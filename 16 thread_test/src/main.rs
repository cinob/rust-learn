use std::thread;
// use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // move关键字 强制闭包获取v的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v)
    });

    // 存在变量中
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // };

    // 保证新线程能在main函数退出前执行完毕
    handle.join().unwrap();
}
