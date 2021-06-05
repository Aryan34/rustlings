// variables4.rs

fn main() {
    // can't apply operations to uninstantiated vars (even if type annotated)
    let x: i32;
    x = 1337
    println!("Number {}", x);
}
