fn main() {
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 => println!("three or four"),
        _     => println!("other"),
    }
}
