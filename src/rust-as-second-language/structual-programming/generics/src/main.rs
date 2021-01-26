// ジェネリクスは、関数やデータ型を任意の型に対して動作するように一般化するときに使える仕組み. 何度も同じような関数やデータ型を定義しなくて済む
fn pair<T, S>(t: T, s: S) -> (T, S) { (t, s) }

fn main() {
    // T = i32, S = f64
    let i = pair(1, 1.0);

    // 型を明示
    let i = pair::<isize, f64>(1, 1.0);

    // T = &str, S = Stringで呼び出す
    let s = pair("str", "string".to_string());
}
