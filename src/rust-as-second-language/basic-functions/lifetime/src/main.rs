fn main() {
    let s = "owned data".to_string();

    // lifetime from '{' to '}'
    {
        // 's'はmoveするのでここでlifetimeが終わる
        // 't'のライフタイムはこのブロックの終わりまで
        let t = s;
    }

    // ライフタイムと参照の関係
    {
        let s = "owned data".to_string();
        // sへの参照を作る. この参照はブロックの最後で死ぬが, 's'の方が長生きする必要
        let ref_s = &s;
        // 例えば以下のように's'のライフタイムをref_sよりも先に終わらせようとするとエラー
        // let t = s; // cannot move out of `s` because it is borrowed
        // println!("{}", ref_s);
    }
}
