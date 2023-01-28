pub fn run(){
    greeting("Hello", "Arpita");
//bind function to variable
let sum = add(5, 7);
println!("Sum is {}",sum );

//clousure - can use variables outside the scope too.
let n3: i32 = 10;
let add_sums = |n1: i32, n2: i32| n1+n2;
println!("C Sum is {}",add_sums(3,3) );
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1+n2
}