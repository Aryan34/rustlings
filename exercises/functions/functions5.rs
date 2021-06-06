// functions5.rs

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

// functions must return an expression, can't return a statement
fn square(num: i32) -> i32 {
    num * num
}
