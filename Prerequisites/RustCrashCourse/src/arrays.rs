// Arrays - fixed length and same data type

use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    numbers[2]=20;
    println!("{:?}", numbers);

    //get single value
    println!("{}", numbers[0]);

    //length of an array
    println!("Len of an array: {}", numbers.len());

    //arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice {:?}", slice)
}