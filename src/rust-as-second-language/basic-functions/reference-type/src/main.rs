fn main() {
    let x = 1;
    let y: &isize = &x;
    let mut a = 1;
    let b = &mut a;
    *b = 2 * *y;
    println!("{}", *b); // => 2
}
