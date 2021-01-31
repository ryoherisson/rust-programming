fn count(a: &mut Option<i32>) {
    match a.as_mut() {
        Some(x) => *x += 1,
        None=>()
    };
}

fn main() {
    // Some: 整数値5をOptionで包む
    let mut a = Some(5);
    count(&mut a);
    println!("{}", a.unwrap());
}
