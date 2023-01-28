// if else

pub fn run(){
let age: u8=18;
let check_id: bool = true;
if age >= 21 && check_id {
    println!("above 21 and id checked");
} else if age<21 && check_id {
    println!("below 21 and id checked");
} else {
    println!("id checking pending");
}
}