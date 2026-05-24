// core:: has basic types that work without OS
// you can use these in no_std environment

fn main() {
    // these all come from core, work in no_std too
    let x: i32   = 42;
    let y: f32   = 3.14;
    let z: bool  = true;
    let a: u8    = 0xFF; // common in embedded for registers

    // core::option::Option works in no_std
    let val: Option<u8> = Some(a);
    match val {
        Some(v) => println!("value = {}", v),
        None    => println!("none"),
    }

    // core::result::Result works in no_std
    let res: Result<u8, &str> = Ok(42);
    match res {
        Ok(v)  => println!("ok = {}", v),
        Err(e) => println!("err = {}", e),
    }
}
