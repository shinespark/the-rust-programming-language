fn main() {
    // 変数束縛
    let v = vec![1, 2, 3];

    // ムーブセマンティクス
    let v = vec![1, 2, 3];
    let v2 = v;

    // 以下はエラー
    //    println!("v[0] is: {}", v[0]);

    let v = vec![1, 2, 3];
    take(v);
    // 以下もエラー
    //    println!("v[0] is: {}", v[0]);

    // 以下はCopyトレイトが実装された i32だからOK
    let v = 1;
    let v2 = v;
    println!("v is: {}", v);

    let a = 5;

    let _y = double(a);
    println!("{}", a);

    let a = true;

    let _y = change_truth(a);
    println!("{}", a);

    // もし毎回すべての関数で所有権を返すなら...
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = foo(v1, v2);
    // 上手くやるには参照と借用をする
}

fn take(v: Vec<i32>) {
    // ここで何が起きるかは重要ではない
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // v1とv2についての作業を行う

    // 所有権と関数の結果を返す
    (v1, v2, 42)
}
