// borrows mutably, can change the value
fn add_surname(name: &mut String) {
    name.push_str(" kumar");
}

fn main() {
    let mut name = String::from("arjun");

    // pass mutable reference with &mut
    add_surname(&mut name);

    // name is changed
    println!("{}", name); // arjun kumar
}
