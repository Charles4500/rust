fn main() {
    // println!("Hello, world!");
    /*
    input: 1 + 2
    output: 3

    Step 1: Check for constraints:
    1<= a, b <=1000
    */

    // let result = solve_me_first(1, 2);
    // println!("{}", result);

    //Challenge 2 --> Staircase
    print_stair_case(6);
}

// fn solve_me_first(a: i32, b: i32) -> i32 {
//     return a + b;
// }

fn print_stair_case(n: i32) {
    for num in 1..=n {
        for _ in 0..(n - num) {
            print!(" ")
        }
        for _ in 0..num {
            print!("#")
        }
        println!()
    }
}
