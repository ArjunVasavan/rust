fn for_over_vec(items: &Vec<&str>) {
    for item in items {
        println!("{}", item);
    }
}

fn main() {
    let fruits = vec!["apple", "banana", "cherry"];
    for_over_vec(&fruits);
}
