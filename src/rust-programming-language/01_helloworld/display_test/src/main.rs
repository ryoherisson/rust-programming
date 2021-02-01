use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
// `Vec`を含む`List`という名の構造体を定義
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        // `v`を介して`vec`をイテレーションし、同時にカウントを
        // `enumerate`で取得する
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            // Try `write!` to see if it errors. If it errors, return
            // the error. Otherwise continue.
            // `write!`を実行し、エラーが生じた場合はerrorを返す。そうでなければ実行を継続する。
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        // 開きっぱなしのブラケットを閉じて、`fmt::Result`の値を返す。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
