fn main() {
    // integers - signed (can be negative)
    let a: i8  = -10;   // 8  bit: -128 to 127
    let b: i16 = -1000; // 16 bit
    let c: i32 = -50000;// 32 bit (default integer)
    let d: i64 = -99999;// 64 bit

    // integers - unsigned (only positive, common in embedded)
    let e: u8  = 255;   // 8  bit: 0 to 255   (register values)
    let f: u16 = 65535; // 16 bit: 0 to 65535
    let g: u32 = 100000;// 32 bit
    let h: u64 = 999999;// 64 bit

    // floats
    let i: f32 = 3.14;  // 32 bit float
    let j: f64 = 3.14159265358979; // 64 bit float

    // boolean
    let k: bool = true;
    let l: bool = false;

    // character
    let m: char = 'A';

    // usize - used for array index and sizes
    let n: usize = 10;

    println!("i8  = {}", a);
    println!("u8  = {}", e);  // most common in embedded
    println!("f32 = {}", i);
    println!("bool= {}", k);
    println!("char= {}", m);
}
