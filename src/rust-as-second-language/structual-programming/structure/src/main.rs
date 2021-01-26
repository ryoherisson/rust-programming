// Uint Struct
struct Dummy;

// Tuple Struct
struct Point(f64, f64);

// struct Name {field: type, ...}
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    // Unit構造体は名前でそのまま初期化
    let dummy = Dummy;

    // Tuple構造体は関数のように初期化
    // 関数として扱うこともできる
    let point = Point(0.0, 0.0);

    // Tuple構造体のフィールドへのアクセス
    let x = point.0;

    // 普通の構造体の初期化
    let black = Color {r: 0, g: 0, b: 0};

    // 普通の構造体のフィールドへのアクセス
    let r = black.r;
}

