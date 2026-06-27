fn main() {
    println!("Hello, world!");
    //Variables, Data types and Operators

    //Variables are initialised by `let` keyword and are by default immutable

    let my_name = "John"; //Inferred
    let x = 23; //Inferred to a type i32

    println!("My name is {} and my age is {}", my_name, x);

    //Mutable
    let mut y = 10;
    y = 20;
    println!("This is the current value in mem {}", y);

    //Data --> Has a rich set of built in data types
    //integers, floating-point nums, booleans, characters and compound types like arrays, tuples and structs

    //Integers -> Can in various sizes e.g i8, i16 ...
    //Floating number --> Represented by f32 and f64 (32 and 64 bit respectively)

    //Operations and expressions in Rust
    let mut age = 5;
    age += 3;

    println!("This is the sum of age now {}", age);

    //Control flow and functions
    if age > 10 {
        println!("You can have a driving licence");
    } else {
        println!("You are still a minor");
    }

    let count = 5;
    for num in 1..=count {
        println!("{}", num)
    }

    //Loop --> loop kw to create and infinite loop that continues till a  condition is met
    // loop {
    //     println!("Enter a command (or 'quit' to exit)");
    //     let mut input = String::new();
    //     std::io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");
    //     let trimmed_input = input.trim();
    //     if trimmed_input == "quit" {
    //         break;
    //     }
    //     print!("You entered: {}", trimmed_input);
    // }

    //Call function in the main file
    test_function(12);
    greet_user(my_name);
    let ans = longest("Charles", "Kibet");
    println!("The answer is {}", ans);
    print_length("John");

    let mut s = String::from("Hello");
    append_worlds(&mut s);
    println!("{}", s); //Print the modified s

    //Structs , Enums and Pattern Matching

    //Defining and using structs in Rust allows creating custom data types with named fields, enabling encapsulation of related data and behaviour into cohesive units

    struct Person {
        name: String,
        age: u32,
        is_student: bool,
    }
    //We can create instances of a struct
    let person_one = Person {
        name: String::from("John"),
        age: 30,
        is_student: false,
    };

    //Accessing struct
    println!("Name: {}", person_one.name);
    println!("Age: {}", person_one.age);
    println!("Is Student: {}", person_one.is_student);

    //Mutables --> Structs allow modification of values after instantiation

    let mut person_two = Person {
        name: String::from("Bob"),
        age: 25,
        is_student: true,
    };

    person_two.age = 26;

    println!("New age of {} is {}", person_two.name, person_two.age); //Name of Bob is 26

    impl Person {
        fn new(name: String, age: u32, is_student: bool) -> Person {
            Person {
                name,
                age,
                is_student,
            }
        }
    }

    let person_3 = Person::new(String::from("Charlie"), 35, false);

    println!("Third person {}", person_3.name);

    //Enumarations(enums) and their uses
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Green,
        Yellow,
    }
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    println!("{:#?} and  {:#?}", red_light, green_light);

    //Pattern
    //Pattern matching is a powerful feature for destructuring enums and match again their variants
    let traffic_light = TrafficLight::Yellow;

    match traffic_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow => println!("Prepare to stop!"),
    } //Prepare to stop!

    //Option --> Handling potentially absent values
}

//Functions
fn test_function(a_number: i32) -> i32 {
    let sum = a_number * a_number;
    println!("This is the multiple {}", sum);
    return sum;
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name)
}

//Ownership , Borrowing and lifetime

//Ownership --> This is rust's feature for managing memory ensuring memory safety without a  need of  a garbage collector
//Each value has a owner and can only be one owner at a time

//Ownership rules:
//1. Each value in Rust has a single owner
//2. Values are dropped automatically if owner goes of out scope
//3. Ownership can be transferred using moves or borrowed using references

//Borrowing allows to temporarily loan a reference to a value without tranfer of ownership
//There are two types of borrowing in Rust:
//Immutable allows multiple readers but no writers. References created with &  are immutable by default

//Mutable Allows one writer and no readers. References created with &mut are mutable

//Lifetimes are annotations that specify the relationship between references in Rust
//Are denoted using single quoted(') and are usually generic params

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

//Rust's ownership plays a vital role in ensuring  thread safety and preventing data races in concurrent  programming

//Immutable --> Read only
fn print_length(s: &str) {
    println!("Length: {}", s.len())
}

//Mutable --> Write
fn append_worlds(s: &mut String) {
    s.push_str(" World");
}

//References are lightweight pointers that refer to values without taking ownership
//Used extensively in Rust to pass data between function s and share data among different parts of the codebase

//76
