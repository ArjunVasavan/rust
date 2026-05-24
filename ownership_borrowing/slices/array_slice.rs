fn sum(data: &[i32]) -> i32 {
    // &[i32] is a slice, works with any size array
    let mut total = 0;
    for val in data {
        total += val;
    }
    total
}

fn main() {
    let readings = [10, 20, 30, 40, 50]; // sensor readings

    // pass whole array as slice
    println!("total = {}", sum(&readings));

    // pass part of array as slice
    // [1..3] means index 1 to 2
    println!("partial = {}", sum(&readings[1..3]));
}
