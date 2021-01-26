fn main() {
    // copyはトレイト
    let x = 1;

    let y = x;
    println!("{}", y); // => 1

    // 数値はcopyな値
    println!("{:?}", x); // => 1

    // &strもstrへの参照なのでcopyな値
    let a = "abc";
    let b = a;
    println!("{}", a); // => abc
    println!("{}", b); // => abc
}
