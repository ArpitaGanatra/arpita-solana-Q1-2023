// Arrays - fixed length and same data type

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers[2]=20;

    //add to vectors
    numbers.push(6);
    numbers.push(7);

    //pop the last number
    numbers.pop();

    println!("{:?}", numbers);

    //get single value
    println!("{}", numbers[0]);

    //length of an vector
    println!("Len of an Vector: {}", numbers.len());

    //vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice {:?}", slice);

    //loop
    for x in numbers.iter(){
        println!("{}", x);
    }

    //loop and modify values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Modified array: {:?}", numbers);
}