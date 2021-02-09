use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

// ジェネリック引数のTは、具体型Stringで埋められ、 ジェネリック引数のEは具体型io::Errorで埋められる
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// ?演算子を使う
// Result値を処理するために定義したmatch式とほぼ同じように動作
// Resultの値がOkなら、Okの中身がこの式から返ってきて、プログラムは継続します。
// 値がErrなら、 returnキーワードを使ったかのように関数全体からErrの中身が返ってくるので、 エラー値は呼び出し元のコードに委譲されます。
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// さらに短くできる

// '?'はResultを返す関数内でしか使用が許可されていない
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Guess方の定義内にError関数を書く
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            // 予想の値は1から100の範囲でなければなりませんが、{}でした
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    // rootはresult directory直下
    let f = File::open("hello.txt");

    // File::openが失敗した理由にかかわらずpanic!
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     //ファイルを作成しようとしましたが、問題がありました
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!(
    //             "There was a problem opening the file: {:?}",
    //             error
    //         )
    //     },
    // };

    // match はいささか冗長 => unwrapが使える
    // let f = File::open("hello.txt").unwrap();

    // unwrapの代わりにexpectでメッセージを出力
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

}