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

//53
