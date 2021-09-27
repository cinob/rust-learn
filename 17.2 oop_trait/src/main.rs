use oop_trait::Draw;

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>
}

impl Draw for SelectBox {
  fn draw(&self) {
    // 绘制
  }
}

fn main() {

}
