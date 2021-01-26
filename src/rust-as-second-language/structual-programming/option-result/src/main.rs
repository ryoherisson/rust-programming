// ハッシュマップからの値の取得など「値があれば返すが見つからなければnil」という処理をするときに使われる
enum Option<T> {
    None,
    Some(T),
}

// 計算が失敗するかもしれないときに使われる型
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello Rust!");
} 

