use std::fmt;
use std::result;
// use std::fmt::Result;
// use std::io::Result as IoResult;

use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// どの名前がスコープ内にありプログラムで使われている名前か、わからない
// testsモジュール持ち込みで使う
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// useでパスをスコープに追加 ≒ ファイルシステムにおいてシンボリックリンクを貼ることと似ている
// 関数がローカルで定義されていないことを明らかにするため、hostingまでを指定
// 一方構造体やenumその他要素をuseで持ち込む時はフルパスで書く
use crate::front_of_house::hosting;

// 外部のコードがhosting::add_to_waitlistを使って, add_to_wait_list関数を呼び出せる
// pub use crate::front_of_house::hosting;

// 相愛パスで要素をスコープに持ち込むこともできる
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
} 


// fn function1() -> fmt::Result {
//     // --snip--
//     // （略）
// }

// fn function2() -> io::Result<()> {
//     // --snip--
//     // （略）
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
