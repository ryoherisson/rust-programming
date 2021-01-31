/*
Result<T,E>は失敗するかもしれない処理の結果を表現する列挙型.
Resultはrustコンパイラから特別扱いされており、無視するとwarinigが出る.
Resultは例外がないrustにおける標準のエラーハンドリング方法. エラーが発生する可能性がある場合は結果にResultを用いるようにする.
*/

fn get_value_bad(v: bool, result: &mut usize) -> usize {
    if v {
        *result = 100;
        0 
    } else {
        1
    }
}

fn get_value_good(v: bool) -> Result<usize,&'static str> {
    if v {
        Ok(100)
    } else {
        Err("error message")
    }
}

fn main() {
    // エラーコードを返す書き方
    // 利用者が戻り値を無視できるのでよく無い
    let mut result = 0;
    if get_value_bad(true, &mut result) == 0 {
        println!("success: {}", result);
    } else {
        println!("failure");
    }

    // rustぽい書き方
    match get_value_good(true) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }
}
