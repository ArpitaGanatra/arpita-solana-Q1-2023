//Primitive string- immutable
// string - growable, heap allocated data structure

pub fn run(){
    //primitive string
    let  hello = "hello";

    //growable string
    let g_hello = String::from("Hello");
    println!("{}", hello);

    //get length
    println!("{}", g_hello.len());

    let mut mut_hello = String::from("Hello ");

    //pudh char
    mut_hello.push('W');

    //push string
    mut_hello.push_str("orld!");
    println!("{}", mut_hello);

    //capcity
    println!("Capacity: {}", mut_hello.capacity());

    //is empty?
    println!("Is Empty: {}", mut_hello.is_empty());

    //contains\
    println!("Contains 'World: {}", mut_hello.contains("World"));

    //replace
    println!("Replace: {}", mut_hello.replace("World", "There"));

    //loop through sring by whitespace
    for word in mut_hello.split_whitespace(){
        println!("{}", word);
    }

    //create a string with certain capacity
    let mut s =String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // match two strings if they are equal
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}