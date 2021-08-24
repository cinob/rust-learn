fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let tup: (i32, f64, u8) = (-500, 50.2, 5);
    // 解构元组
    let (a, b, _c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, tup.2);

    let arr = [1, 2, 3, 4];
    println!("arr: {}", arr[0]);
    let same = [6; 3]; // 等同于 [6, 6, 6]
    println!("arr: {}", same[2]);
    another_func(12);

    let y = {
        let x = five();
        x + 1
    };
    println!("y: {}", y);
}

fn another_func(x: i32) {
    println!("Another function {}", x);
}

fn five() -> i32 {
    5
}