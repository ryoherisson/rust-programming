fn main() {
    let a : Result<i32, _> = "4963".parse();
    let b : Result<i32, _> = "hao123".parse();
    match a {
        Ok(x)=>println!("{}", x),
        Err(_)=>println!("変換に失敗")
    };
    match b {
        Ok(x)=>println!("{}", x),
        Err(_)=>println!("変換に失敗")
    };
}
