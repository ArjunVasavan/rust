fn main() {
    let mut n = 10;
    while n > 0 {
        n-=1;

        if n == 5 {
            println!("Skipping 5!");
            continue;
        }

        println!("Number => {:?}",n);
    }
}
