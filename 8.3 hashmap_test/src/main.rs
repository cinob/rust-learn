use std::collections::HashMap;

fn main() {
    // 所有的键必须拥有相同的类型，所有的值也必须拥有相同的类型
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 50);
    println!("scores {:#?}", scores);

    let teams = vec![String::from("blue"), String::from("green")];
    let initail_scores = vec![10, 50];
    let scores2: HashMap<_,_> = teams.iter().zip(initail_scores).collect();
    println!("scores2 {:#?}", scores2);

    // 访问HashMap中的值
    println!("Blue team's scores: {:#?}", scores.get(&String::from("blue")));
    // 无特定顺序
    for (key, val) in &scores2 {
        println!("{}: {}", key, val);
    }

    // 覆盖旧值
    scores.insert(String::from("blue"), 25);
    println!("Blue team's new scores: {:#?}", scores.get(&String::from("blue")));
    // 判断是否存在某个键
    let has = scores.entry(String::from("yellow"));
    println!("Is has the yellow ? {:#?}", has);
    // 不存在则赋值
    has.or_insert(80);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // 解引用
        *count += 1;
    }
    println!("words: {:#?}", map);


    // HashMap中的所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 报错: 所有权已经转移给了HashMap
    // println!("field_name: {}, field_value: {}", field_name, field_value);
}
