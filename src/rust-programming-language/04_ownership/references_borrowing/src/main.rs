fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // let s = String::from("hello");
    let mut s = String::from("hello");
    change(&mut s);
    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
    let r2 = &mut s;

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！=>エラーにならない.
    // println!("{}", r3);

    // let reference_to_nothing = dangle();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }