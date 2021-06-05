// variables2.rs

fn main() {
    // can't apply operations to uninstantiated vars
    let x = 7;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
