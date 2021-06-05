// variables3.rs

fn main() {
    // vars are immutable by default, instantiate them as mutable with "mut" keyword
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
