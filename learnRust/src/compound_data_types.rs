//Compound data types in rust
//arrays, tuples,slices and string(slice string)

fn main() {
    //Arrays --> Contains only data of the same type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array : {:?}", fruits);
    println!("First fruit  : {}", fruits[0]);
    println!("Second fruit : {}", fruits[0]);
    println!("Third fruit : {}", fruits[0]);

    //Tuples
    let human: (&str, i32, bool) = ("Alice", 30, false);
    println!("Human Tuple : {:?}", human);
    let my_mix_tuple = ("Biegon", 23, true, [1, 2, 3, 4]);
    println!("My mix Tuple {:?}", my_mix_tuple);

    //Slices:[1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Dog"];
    println!("Number Slice {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"Learn Rust".to_string(),
        &"The Rust programming language".to_string(),
    ];
    println!("Book Slice {:?}", book_slices);
}
