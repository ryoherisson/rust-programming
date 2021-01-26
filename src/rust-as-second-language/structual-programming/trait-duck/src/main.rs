// ゼロコスト
// trait trait-name {..} interfaceみたいなもの
trait DuckLike {
    fn quack(&self);

    fn walk(&self) {
        println!("walking");
    }
}

struct Duck;

impl DuckLike for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

struct Tsuchinoko;

impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        println!("mew");
    }

    fn walk(&self) {
        println!("wriggling");
    }
}

impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self { // *self = 3
            println!("quack");
        }
    }
}

// トレイト境界

// ジェネリクスの型パラメータに`型パラメータ名: トレイト名`で境界をつけることができる
fn duck_go<D: DuckLike>(duck: D) {
    duck.quack();
    duck.walk();
}

fn main() {
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    duck.quack(); // => quack
    tsuchinoko.quack(); // => mew
    tsuchinoko.walk(); // wriggling
    i.quack(); // => quack; quack; quack

    duck_go(duck);

    let f = 0.0;
    // duck_go(f); DuckLikeを実装していない型は渡せない
}
