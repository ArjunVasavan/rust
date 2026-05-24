// takes ownership of the string
fn print_name(name: String) {
    println!("name = {}", name);
    // name is dropped here
}

fn main() {
    let name = String::from("arjun");

    // name moves into function
    print_name(name);

    // this would ERROR: println!("{}", name);
    // because name was moved into function
}
