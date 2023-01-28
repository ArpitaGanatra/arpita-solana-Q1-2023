// rust variables are immutable by default
//rust is a block scoped language
pub fn run(){
    // to make variable mutable, use mut keyword
    let mut age=23;
 let name="Arpita";
 println!("My name is {} and I am {}", name, age);
 age=24;
 println!("My name is {} and I am {}", name, age);


 //define constants
 //constants should compulsarily have a type
 const ID: i32 = 001;
 println!("ID is: {}", ID);


 //asign multiple variable at once
 let (my_name, my_age) = ("Arpita", 23);
 println!("My name is {} and I am {}", my_name, my_age);

}