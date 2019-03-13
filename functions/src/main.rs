fn main() {
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
