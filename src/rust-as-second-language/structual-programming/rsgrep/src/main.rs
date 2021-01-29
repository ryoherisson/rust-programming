// regex
extern crate regex;
// Regexをインポート
use regex::Regex;

// stdクレートのfsモジュールにあるFile型
use std::fs::File;
// {}でまとめてインポート
use std::io::{BufReader, BufRead};
// モジュール自体をインポート
use std::env;

fn usage() {
    println!("regrep PATTERN FILENAME")
}

fn main() {
    // 引数からpatternを引き出す
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };
    // パターンからRegexを改めて作る
    // 無効な正規表現だった場合はエラー
    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("invalid regexp {}: {}", pattern, e);
            return
        }
    };

    // envモジュールのargs関数でプログラムの引数を取得
    // 2番目をnthで取得（0番目はプログラムの名前、1番目はパターンで今は無視）
    // 引数があるかわからないのでOptionで返される
    let filename = match env::args().nth(2) {
        Some(filename) => filename,
        None => {
            usage();
            return;
        }
    };

    // File構造体のopen関連関数でファイルを開く
    // 失敗する可能性があるのでResultで返される
    // 下の方でfilenameをつかうため、filenameは&参照
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };

    // Fileのままは遅い+linesメソッドを使うためにBufReaderに包む
    let input = BufReader::new(file);
    // Bufreaderが実装するトレイトのBufReadにあるlinesメソッドを呼び出す
    // 返り値はイテレータなのでfor式が繰り返しできる
    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };

        // パターンにマッチしたらプリント
        // is_matchはリードオンリーなので参照型
        if reg.is_match(&line) {
            println!("{}", line);
        }
    }
}
