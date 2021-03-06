use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn sorted_string(s: &str) -> String {
    // s.chars()で文字のイテレータを取り出し、collectでベクトルにする
    // sはVec<char>型. <_>は省略記法
    // into_iter()で文字のイテレータから文字列を作成. 文字のベクトルを文字列に変化
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.into_iter().collect::<String>()
}

// Anagram構造体
struct Anagram(HashMap<String, Vec<String>>);

impl Anagram {
    // トレイト境界`AsRef<Path>`は、ざっくり意訳すると「パス名っぽいもの」を表す
    // `Self`は、`Anagram`へのエイリアス
    // 「計算に成功したらその値を取り出し、失敗したらResultのErrで関数から抜ける」という処理が必要な場合は、?後置演算子が使える.
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        let file = File::open(dictfile)?; // ★
        let file = io::BufReader::new(file);
        // ハッシュマップ
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?; // ★
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    // テーブルを更新するので`&mut self`を使う
    // 登録した単語をテーブルが所有するので、`word`の所有権も奪う
    fn add_word(&mut self, word: String) {
        // ソートを作ってキーにする
        let sorted = sorted_string(&word);
        // キーに対応する値があればそれを、なければ新たにデフォルト値（Vec::new()）を入れる
        // 返り値はキーに対応する値
        // ハッシュマップはデータの所有者なので、キーもデフォルト値も所有権を奪う
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    // 検索はリードオンリーなので`&self`を使う
    // キーはリードオンリーなので`word`も参照で受け取る
    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        // データの所有権はハッシュマップにあるので、返り値は参照型
        // 参照型なのでコピーは発生せず、高速
        self.0.get(&word)
    }
}

fn main() {
    // コマンドライン引数として単語を受け取る
    let word = std::env::args().nth(1).expect("USAGE: word");
    // 辞書からAnagram構造体を作る
    // 多くのUnix環境ではパスに辞書がある
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");
    println!("{:?}", table.find(&word));
}
