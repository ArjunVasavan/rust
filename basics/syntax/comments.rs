fn main() {
    // this is a single line comment
    // anything after // is ignored by compiler

    let x = 5; // comment can be at end of line too

    /*
        this is a multi line comment
        useful for longer explanations
        compiler ignores everything between /* and */
    */

    let y = 10;

    // /// is doc comment - used for documentation
    /// this function adds two numbers
    // (doc comments are usually above functions/structs)

    println!("x = {}", x);
    println!("y = {}", y);

    // commenting out code temporarily
    // let z = 99;         // this line is disabled
    // println!("{}", z);  // this line is disabled
}
