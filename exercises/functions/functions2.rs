// functions2.rs

fn main() {
    call_me(3);
}

// type annotations required for function arguments
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
