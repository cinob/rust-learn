fn main() {
    let number_list = vec![30, 40, 26, 58, 79];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['f', 'a', 'h', 'w', 'j'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);

    println!("The largest number is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
