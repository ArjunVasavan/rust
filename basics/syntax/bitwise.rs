fn main() {
    // & AND - both bits must be 1
    let a: u8 = 0b1100_1010;
    let b: u8 = 0b1111_0000;
    let result = a & b;
    println!("AND = {:08b}", result); // 1100_0000

    // | OR - at least one bit must be 1
    let result = a | b;
    println!("OR  = {:08b}", result); // 1111_1010

    // ^ XOR - bits must be different
    let result = a ^ b;
    println!("XOR = {:08b}", result); // 0011_1010

    // ! NOT - flip all bits
    let result = !a;
    println!("NOT = {:08b}", result); // 0011_0101

    // << left shift - shift bits left by n
    let result = a << 1;
    println!("<<1 = {:08b}", result); // shifts left by 1

    // >> right shift - shift bits right by n
    let result = a >> 1;
    println!(">>1 = {:08b}", result); // shifts right by 1
}
