fn main() {
    println!("Hello, world!");

    // 変数束縛
    let x = 5;

    // パターン
    let (y, z) = (1, 2);

    // 型アノテーション
    let a: i32 = 5;

    // 可変性
    let mut b = 5;
    b = 10;

    // 束縛を初期化する
    //    let c: i32;
    //    println!("Hello, world! c is: {}", c);

    // スコープとシャドーイング
    let d: i32 = 8;
    {
        println!("{}", d); // "8"を表示する
        let d = 12;
        println!("{}", d); // "12"を表示する
    }
    println!("{}", d); // "8"を表示する
    let d = 42;
    println!("{}", d); // "42"を表示する

    let mut e: i32 = 1;
    e = 7;
    let e = e; // xはイミュータブルになって7に束縛されました

    let f = 4;
    let f = "I can also be bound to text!"; // fは違う型になりました

    // 4.2 関数
    print_number(5);
    print_sum(5, 6);

    // 早期リターン

    // 発散する関数
    //    diverges;

    // 関数ポインタ
    // 型推論なし
    let g: fn(i32) -> i32 = plus_one;

    // 型推論あり
    let h = plus_one;
    let six = g(5);
    let six = h(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
    // ! は発散する(diverges)
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
