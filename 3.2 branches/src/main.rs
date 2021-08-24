fn main() {
    let arr = [1, 2, 3, 4, 5];
    method1(arr);
    method2(arr);
    method3(arr);
}


fn method1(data: [i32; 5]) {
    let mut index = 0;
    loop {
        if index >= data.len() {
            break;
        }
        println!("value is {}", data[index]);
        index += 1;
    };
}

fn method2(data: [i32; 5]) {
    let mut index = 0;

    while index < data.len() {
        println!("value is {}", data[index]);
        index = index + 1;
    }
}

fn method3(data: [i32; 5]) {
    for val in data.iter() {
        println!("value is {}", val);
    }
}
