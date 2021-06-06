// functions3.rs

// input must be provided in function call for all defined arguments 
fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
