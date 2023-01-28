// used to create custom data types

//traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

//Tuple struct
// struct Color(u8, u8, u8);

//struct with functions
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct Person
    fn new(first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //get full name
    fn get_full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //modify
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }
}

pub fn run(){
//  let mut c = Color {
//     red: 255,
//     green: 0,
//     blue: 0,
//  };

//  //modufy value
//  c.red =200;

//  println!("Color: {} {} {}", c.red, c.green, c.blue);

// let mut c = Color(255, 0, 0);
// println!("Color: {} {} {}", c.0, c.1, c.2);
let mut p = Person::new("Arpita", "Ganatra");
println!("{} {}", p.first_name, p.last_name);

p.set_last_name("Williams");
println!("{} {}", p.first_name, p.last_name);
println!("{}", p.get_full_name());
}