fn main() {
    // 创建一个新字符串
    let mut s1 = String::new();
    // 使用to_string方法基于字符串字面量创建String
    let mut s2 = "wahahah".to_string();
    // 使用String::from基于字符串字面量创建String
    let s3 = String::from("hahahah");

    // 更新字符串
    s2.push_str(&s3);
    s2.push('o');
    println!("s2: {}", s2);

    // 字符串合并
    // +
    let s4 = "hello,".to_string() + &s3;

    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");
    // format!宏 适用于比较复杂的字符串合并
    let sa = format!("{}-{}-{}", s5, s6, s7);
    println!("s4: {}; sa: {}", s4, sa);

    let len = String::from("加油").len();
    println!("len: {}", len);

    println!("chars: {:#?}", String::from("加油").chars());
    println!("bytes: {:#?}", String::from("加油").bytes());
}
