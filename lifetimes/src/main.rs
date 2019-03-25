fn main() {
    // 黙示的に
    fn foo(x: &i32) {
    }

    // 明示的に
    fn bar<'a>(x: &'a i32) {
    }

    // 'a: ライフタイムaと読む


    // implブロック
    let y = &5; // これは`let _y = 5; let y = &_y;`と同じ
    let f = Foo { x: y };

    println!("x is: {}", f.x());


    // 'static
    let x: &'static str = "Hello, world.";
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}
