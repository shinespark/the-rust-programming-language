fn main() {
    // 前回のおさらい
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    // v1, v2をそのまま使い続ける為に所有権を返却するように愚直に書くならこう
    let (v1, v2, answer) = foo(v1, v2);

    // 参照を利用すればこう書ける
    let answer = foo2(&v1, &v2);

    // &mut参照
    // {} を削除すればエラー
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x); // 6

    let mut x = 5;

    let y = &mut x; // -+ xの&mut借用がここから始まる
    *y += 1;
    println!("{}", x); // -+ - ここでxを借用しようとする
                       // -+ xの&mut借用がここで終わる

    let mut x = 5;
    {
        let y = &mut x; // -+ &mut借用がここから始まる
        *y += 1;
    } // -+ ... そしてここで終わる
    println!("{}", x); // <- ここでxを借用しようとする

    // 借用が回避する問題

    // イテレータの無効
    // ↓はOK
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    // ↓はエラー
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        v.push(34);
    }

    // 解放後の使用
    // ↓はエラー
    let y: &i32;
    {
        let x = 5;
        y = &x;
    }
    println!("{}", y);

    // ↓もエラー
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // v1とv2についての作業を行う

    // 所有権と関数の結果を返す
    (v1, v2, 42)
}

fn foo2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // 答えを返す
    42
}
