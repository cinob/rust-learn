// struct ImportantExcerpt<'a> {
//     part: &'s str
// }

fn main() {
    let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let res = longest(string1.as_str(), string2);

    let res;
    {
        // let string2 = "xyz";
        let string2 = String::from("xyz");
        res = longest(string1.as_str(), string2.as_str());
    }

    println!("longest: {}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 使用参数x的生命周期
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }