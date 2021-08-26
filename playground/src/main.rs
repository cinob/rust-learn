use std::collections::HashMap;

fn main() {
    let arr = vec![20, 2, 52, 35, 15, 20];
    let len = arr.len();

    let mut sum = 0;
    for n in arr.iter() {
        sum += n;
    }

    let mid = match len % 2 {
        0 => {
            arr[len / 2]
        },
        1 => {
            (arr[(len - 1) / 2] + arr[(len + 1) / 2]) / 2
        },
        _ => 0
    };

    let mut map = HashMap::new();
    for n in arr.iter() {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    println!("sum is {}, mid is {}, map is {:#?}", sum, mid, map);
}
