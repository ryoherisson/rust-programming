#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    // #2
    // let rect1 = (width1, height1);

    let rect1 = Rectangle {width: 30, height: 50};

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:?}", rect1);
    
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    println!("rect1 is {:#?}", rect1);
}

// #1
// 引数には関連があるが、そのことがプログラム内のどこにも表現されていない。
// 一緒のグループにする方が読みやすく扱いやすい => タプル使う
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// #2
// マシに見えるが、タプルのどちらがwidth, heightかわからない. 覚えておかないといけない.
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// #3 
// 構造体を使う
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}