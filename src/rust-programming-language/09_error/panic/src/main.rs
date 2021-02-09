fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
    // RUST_BACKTRACE=1 cargo runで何が起きエラーが発生したのかのバックトレースが得られる.
}
