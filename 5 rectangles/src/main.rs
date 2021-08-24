#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// 多个impl块
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20
    };
    let rect3 = Rectangle::square(60);

    // println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
