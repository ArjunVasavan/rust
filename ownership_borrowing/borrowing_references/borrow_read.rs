// borrows the string, does not take ownership
fn print_name(name: &String) {
    println!("name = {}", name);
    // name is NOT dropped here, just borrowed
}

fn main() {
    let name = String::from("arjun");

    // pass reference with &
    print_name(&name);

    // name still valid here because we only borrowed
    println!("still valid: {}", name);
}
