fn main() {
    let x = 5;
    // x = 6; // エラー!


    let mut x = 5;
    x = 6; // 問題なし!

    // ミュータブル参照
    let mut x = 5;
    let y = &mut x;
}
