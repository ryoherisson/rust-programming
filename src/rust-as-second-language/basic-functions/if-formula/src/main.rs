fn factorial(n: usize) -> usize {
    // ifは式なので関数の最後に置くと値を返せる
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    // ';'をつけることで文になり、コンパイルが通る
    factorial(10);
    println!("{}", factorial(10));
}
