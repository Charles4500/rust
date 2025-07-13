//Primitive data types in rust
//It is statically types programming language
fn main() {
    //println!("Hello, world!")
    let x: i32 = 42; //32 bits of data

    let y: u64 = 100; //64bits of data

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    //FLoats
    //f32 f64

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    //Boolean values
    //true or false
    let is_snowing: bool = true;
    println!("Is is snowing ? {}", is_snowing);

    //Character type - char
    let letter: char = '@';
    println!("First letter of alphabet: {}", letter);
}
