fn main() {
    // let x = 5; error: cannot assign twice to immutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    let y = 5;
    println!("y: {}", y);
    let y = y + 5;
    println!("y+5: {}", y);
    let y = y + 5;
    println!("y+10: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {} end", spaces);
}
