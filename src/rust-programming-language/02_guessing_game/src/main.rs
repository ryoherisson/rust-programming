// 「標準ライブラリ」「変数」「型の関数」「標準入力」「参照」「パニック処理」「クレート」「トレイト」「matchフロー制御演算子」「シャドーイング」「loop」
// https://news.mynavi.jp/series/rust/
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲーム");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("シークレットナンバーはこれ: {}", secret_number);

    loop {
        println!("どの数だと思う？ = ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込み失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess: u32 = guess // シャドーイング
        //     .trim()
        //     .parse()
        //     .expect("数を入力してください！");

        println!("入力値: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎです！"),
            Ordering::Greater => println!("大きすぎです！"),
            Ordering::Equal => {
                println!("そのとおり！");
                break;
            }
        }
    }
}
