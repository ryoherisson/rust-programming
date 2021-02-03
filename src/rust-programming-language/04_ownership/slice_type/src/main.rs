fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error!    (エラー！)
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // 文字列リテラルは、すでに文字列スライス*な*ので、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
}



// &strでStringとstrどちらも使えるようになる
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// wordの値が有効に使えないケース
// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // wordの中身は、値5になる

//     s.clear(); // Stringを空にする。つまり、""と等しくする

//     // wordはまだ値5を保持しているが、もうこの値を有効に使用できる文字列は存在しない。
//     // wordは完全に無効なのだ！
// }


// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }