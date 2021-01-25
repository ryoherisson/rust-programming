
// fn main() {
//     let x: i32 = 1 + 2;
//     let mut y = x;
//     y = 5;
//     println!("y: {}", y);
//     let z = y;
//     println!("z: {}", z);

//     let x: &str = "hoge";
//     println!("x: {}", x);
// }


fn rebind() {
    let sum = 0;
    for i in 0..10 {
        let sum = sum + i;
    }
    println!("{}", sum); // => 0
}

fn reassign() {
    let mut sum = 0;
    for i in 0..10 {
        sum = sum + i;
    }
    println!("{}", sum); // => 45
}

fn main() {
    rebind();
    reassign();
}