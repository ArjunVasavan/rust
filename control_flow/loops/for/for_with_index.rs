fn for_with_index(items: &Vec<&str>) {
    for (i, item) in items.iter().enumerate() {
        println!("{}: {}", i, item);
    }
}

fn main() {
    let fruits = vec!["apple", "banana", "cherry"];
    for_with_index(&fruits);
}
