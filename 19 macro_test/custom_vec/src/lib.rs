// 表示这个宏会在它所处的包被引入作用域后可用。
// 缺少这个标注的宏则不能被引入作用域
#[macro_export]
// 定义宏 macro_rules! + name
macro_rules! vec {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x)
            )*
            temp_vec
        }
    };
}