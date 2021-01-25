fn main() {
    let x = 12;
    match (x) {
        0 => println!("0"),
        1...10 => println!("small number"),
        n => println!("big number: {}", n),
    }

    match (1.0, 1) {
        // tuple pattern
        (0.0, 0) => println!("all zero"),
        // variable pattern
        (f, 0...10) => println!("float: {} with small number", f),
        // variable
        _ => println!("other tuple"),
    }
}
