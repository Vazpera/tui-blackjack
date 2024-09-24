use std::fmt::Display;

struct Displayable<T>
where
    T: Display,
{
    value: T,
}

fn main() {
    println!("Hello, world!");
}
