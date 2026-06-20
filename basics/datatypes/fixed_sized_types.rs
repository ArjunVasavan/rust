 /* an function that wheen we add negative returns error */

fn get_default_age()->i8 {
    return -1;
}

fn main() {
    let a: u32 = 25;
    let b: u8 = 64;
    println!("unsigned 32 bit {}",a);
    println!("unsigned 8 bit {}",b);
    let age: i8 = get_default_age(); /* if you add u8 inspite of i8 its an error */
    println!("age is {}",age);
}
