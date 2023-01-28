pub fn run(){
//print to console
println!("hello world form the print rs file");

//Basic Formatting
// println!("Num: {}", 1)
println!("{} is from {}", "Arpita", "India");

//Positio"nal parameters - use arpita 2 times in a sentence.
println!("{0} is from {1}  and {0} likes to {2}", "Arpita", "India", "code");

//Named arguments
println!("{name} likes to {activity}", name="Arpita", activity="code");

//Plaeholder traits
println!("binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

//placeholder for debug traits
println!("{:?}", (10, true,"Arpita"));

//basic math
println!("10+10={}", 10+10);
}