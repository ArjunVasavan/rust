fn main() {

    let numbers: [i32;5] = [1,2,3,4,5];

    println!("Numbers Array: {:?}",numbers);

    // let mix: [1,2,"apple",true];
    
    let fruits: [&str;3] = ["Apple","Banna","Orange"];

    print!("fruits: {}\n",fruits[0]);
    print!("fruits: {:?}\n",fruits);
 // tuple
    let human: (String, i32, bool) = ("Alice".to_string(),30,false) ;

    println!("Human tuple {:?}\n",human);
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);

    println!("My Mix Tuble: {:?}\n",my_mix_tuple);

 // slices
    // [1,2,3,4] -> its an contigiuous lines of sequence
    // eg: 

    let number_slices:&[i32] = &[1,2,3,4,5];

    println!("Number SLices: {:?}\n",number_slices);

    let animal_slices:&[&str] = &[ "Lion","Elephant","Crocodile" ];

    println!("Animals: {:?}",animal_slices);


}
