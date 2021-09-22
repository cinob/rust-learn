use std::mem::drop;
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPonter with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    let e = CustomSmartPointer { data: String::from("some data")};
    println!("CustomSmartPointers created.");
    // 手动清理值
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}
