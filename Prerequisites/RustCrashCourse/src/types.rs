// Primitive Types 
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits take in the memory)
// Float: f32, f64
// Boolean (bool)
// Characters (char) (only 1 character and not strings)
// Tuples (list)
// Array (of fixed length, and not vectors)

//rust is staticlly typed language, so compiler needs to know the memory of each var beforehand.
//bt the compiler here can usually infer the type of the variable

pub fn run() {
    //default is i32
    let x=1;

    //for float, default is f64
    let y=2.5;

    //add explicit type
    let z: i64 = 4545554545;

    //find max size
    println!("Max value of i32: {}", std::i32::MAX);
    println!("Max value of f64: {}", std::f64::MAX);

    //bool
    let is_active=true;

    //get bool from expressions
    let is_greater = 10>5;

    //character - use 'single quote' 
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, is_greater, a1,face));
}