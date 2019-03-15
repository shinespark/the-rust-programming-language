fn main() {
    let x = 5;

    if x == 5 {
        println!("x は 5 です!")
    } else if x == 6 {
        println!("x は 6 です!")
    } else {
        println!("x は 5 ではありません :(")
    }

    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("y: {}", y);
}
