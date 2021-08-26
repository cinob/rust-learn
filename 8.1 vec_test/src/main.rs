// 动态数组的容量是分配给未来添加到动态数组上的任何元素的空间量。
// 不要与动态数组的长度相混淆，后者规定了动态数组中实际元素的数量。
// 如果一个动态数组的长度超过了它的容量，它的容量将自动增加，但它的元素将不得不被重新分配。

// 例如，一个容量为10、长度为0的动态数组将是一个空的动态数组，其空间可以再容纳10个元素。
// 把10个或更少的元素推到动态数组上不会改变它的容量或导致重新分配。
// 然而，如果动态数组的长度增加到11，它将不得不重新分配，这可能很慢。
// 由于这个原因，建议尽可能使用Vec::with_capacity来指定动态数组的预期大小。

fn main() {
    // 创建动态数组
    let v1: Vec<i32> = Vec::new();
    // 使用vec!宏, 创建包含值的动态数组
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    println!("v2 is {:#?}", v2);

    // 访问动态数组 索引, get
    let third = &v2[2];
    println!("The third element is {}", third);
    match v1.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
    // 访问越界元素， 编译报错
    // let does_not_exist = &v1[100];
    // 返回None
    let does_not_exist = v1.get(100);
    println!("does_not_exist is {:?}", does_not_exist);

    // 在存在指向动态数组元素的引用时,尝试向动态数组中添加元素
    let mut v3 = vec![1, 2 , 3];
    let first = &v3[0];
    // 报错: 动态数组连续存储的特性, 可能没有足够的空间插入的新元素,
    // 这就需要分配新的内存空间，并将旧的元素移动到新的空间上。
    // 从而导致first的引用指向被释放的内存
    // v3.push(4);
    println!("The first element is {}", first);

    // 遍历
    for i in &mut v2 {
        println!("{}", i);
        // *解引用
        *i += 50;
    }
    println!("v2 is {:#?}", v2);

    // 使用枚举来存储多个类型的值
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.20),
        SpreadsheetCell::Text(String::from("hahahaha"))
    ];
    println!("The row is {:#?}", row);
}
