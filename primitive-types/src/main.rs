fn main() {
    // ブーリアン型
    let x = true;
    let y: bool = false;

    // char型
    let c = 'c';
    let two_hearts = '💕'; // 4バイト

    // 数値型
    // i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64,
    // u: 符号なし, i: 符号あり, fは浮動小数点型
    // isizeとusizeは可変長

    // 配列
    let a = [1, 2, 3];
    let mut m = [1, 2, 3];

    // 0で初期化した長さ20の配列
    let a = [0; 20];
    println!("a has {} elements", a.len());

    // 添字記法
    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);

    // スライシング構文
    let a  = [0,1,2,3,4];

    let complete = $a[..]; // すべてのスライス
    let middle = $a[1..4]; // 1, 2, 3のみを要素に持つスライス

    // str型
    // https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/strings.html
    // https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/references-and-borrowing.html

    // タプル
    let x = (1, "hello");
    // タプル(型注釈付き)
    let x: (i32, &str) = (1, "hello");

    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)
    x = y;

    // タプルの分配
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    (0,); // 1要素のタプル
    (0); // 丸括弧に囲まれたゼロ

    // タプルのインデックス
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);

    // 関数
    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;
}
