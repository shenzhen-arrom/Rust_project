fn main() {
    let num = Some(4);
    match num {
        Some(x) if x<5 => println!("<5"),
        Some(x) => println!(" x: {}" ,x),
        None =>(),
    };
}