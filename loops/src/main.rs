fn main() {
    // loop
    loop {
        println!("Loop forever!");
    }

    // while
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // while true よりは loop が好ましい

    // for
    for x in 0..10 {
        println!("{}", x);
    }

    // 列挙
    // 対象: レンジ
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    // 対象: イテレータ
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}, {}", linenumber, line);
    }

    // 反復の早期終了
    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            break;
        }
    }

    // ループラベル
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // x のループを継続
            if y % 2 == 0 {
                continue 'inner;
            } // y のループを継続
            println!("x: {}, y: {}", x, y);
        }
    }
}
