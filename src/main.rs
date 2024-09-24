pub mod funcs;
use funcs::add;
use funcs::sub;

fn main() {
    println!("1+2={}", add(1, 2));
    println!("1-2={}", sub(1, 2));
}
