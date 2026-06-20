fn main() {
    println!("Hello, world!");
    /*
    input: 1 + 2
    output: 3

    Step 1: Check for constraints:
    1<= a, b <=1000
    */

    let result = solve_me_first(1, 2);
    println!("{}", result);
}

fn solve_me_first(a: i32, b: i32) -> i32 {
    return a + b;
}
