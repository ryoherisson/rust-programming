#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体上にmethodを作る
// impl(implementation)
// implブロックを複数に分けることもできる
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // &selfでないとすでにmoveしているのでerror
    println!("{}", rect1.area());

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 関連関数の呼び出し
    let sq = Rectangle::square(3);
}