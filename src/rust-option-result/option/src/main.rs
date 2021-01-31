// Option<T>型は 取得できないかもしれない値 を表現する列挙型.
// 値が無いことを示すNoneとあることを示すSome(T)のどちらかをとる.

fn get_value_bad(v: bool, result: &mut usize) -> bool {
    if v {
        *result = 100;
        true
    } else {
        false
    }
}

fn get_value_good(v: bool) -> Option<usize> {
    if v {
        Some(100)
    } else {
        None
    }
}

fn main() {
    // rustぽくない書き方
    let mut result = 0;
    if get_value_bad(true, &mut result) {
        println!("success: {}", result);
    } else {
        println!("failure");
    }

    // rustぽい書き方
    match get_value_good(true) {
        Some(result) => println!("success: {}", result),
        None => println!("failure"),
    }
}