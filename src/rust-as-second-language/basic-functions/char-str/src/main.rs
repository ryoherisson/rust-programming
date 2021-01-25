fn main() {
    // &strはto_string()メソッドでStringにできる
    let mut a: String = "abc".to_string();
    // String + &str => String
    // &str + String => x, String + String => x
    a += "def";
    println!("{}", a); // => abcdef

    let x = 1.0.to_string();
    println!("{}", x); // 1

    // String => &strにはas_str()
    a += x.as_str();
    println!("{}", a);
}
