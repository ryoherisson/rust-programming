fn main() {
    // let x = 5; error: cannot assign twice to immutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
