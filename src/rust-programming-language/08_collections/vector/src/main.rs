enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];
    let v2 = vec!['a', 'b', 'c'];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];
    
        // vで作業をする
    
    } // <- vはここでスコープを抜け、解放される

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // Noneを返す

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); //  同一スコープ上では、可変と不変な参照を同時には存在させられない

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", *i);
    }

    // 異なる方のベクタを格納したいとき.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
