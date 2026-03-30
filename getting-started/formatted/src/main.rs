fn main() {
    println!("Hello world");
    println!("I'm a Rustacean");

    //Comments

    //Rust supports a few different varieties:
    // ·Line comments which start with //
    // ·Block comments /*...*/
    let x = 5 + /* 90 */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // In general, the `{}` will be automatically replaced with
    // any agurments
    println!("{} days", 31);

    //Positional arguments can be used. Specifiying an integer inside `{}`
    //Determines which additional argument will be replaced. Arguments start
    //at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0} and {0} and  {1} this is {2}" , "Alice", "Bob", "Charles");

    println!("{subject} {verb} {object}",
             object="The lazy dog",
             subject="The quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    // println!("Base 10:   {}", 69420);
    // println!("Base 2 (binary):   {:b}", 69420);
    // println!("Base 8 (octal):   {:o}", 69420);
    // println!("Base 16 (hexadecimal):   {:x}", 69420);

     // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    //println!("{number:>5}", number=1);

      // You can pad numbers with extra zeroes,
    //println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    //println!("{number:0<5}", number=1); // 10000

     // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    let number: f64 =1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}
