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
}
