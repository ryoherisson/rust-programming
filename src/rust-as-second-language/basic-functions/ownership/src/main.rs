fn print_string(s: String) {
    println!("{}", s);
    // s箱の関数の終わりで消滅
    // このタイミングでsのメモリも自動で解放
}

fn ref_string(s: &String) {
    println!("{}", s);
}

fn refmut_string(s: &mut String) {
    // sに変更加えることが可能
    println!("{}", s);
}

fn main() {
    let s = "this is a resource".to_string();
    // 's'が束縛されている文字列の所有権が't'に移る(move). 以降's'は使えない．
    let t = s;
    // println!("s: {}", s); // error: value borrowed here after move
    print_string(t);
    // print_string(t); // error: value used here after move

    // immutableな貸し出し
    let s = "this is a resource".to_string();
    let t = &s;
    ref_string(&s);
    print_string(t.to_string());
    let x = &s;
    print_string(x.to_string());
    // print_string(s); // t野崎はだめ
    print_string(t.to_string());
    print_string(s);

    // mutable
    let mut s = "this is a resource".to_string();
    // mutable参照一つ目
    let t = &mut s;
    print_string(t.to_string());
    // print_string(s); // OK
    // mutable参照2つ目はエラー
    // refmut_string(&s) // cannnot borrow 's' as muable more than once at a time


    // mutableとimmutableな参照は共存できない
    // 無限ループになる
    // let mut vec = vec![1, 2, 3];
    // for i in vec.iter() {
    //     vec.push(i * 2);
    // }

    
}
