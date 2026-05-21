fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5]; 

    let mut powers = 0;

    for num in numbers {
        println!("{num}: {:?}",num.pow(2));
        powers+=num.pow(2);
    }
}
