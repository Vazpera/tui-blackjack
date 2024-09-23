use std::fmt::Display;

#[derive(Debug)]
struct MyStruct<T> {
    value: T,
}

fn main() {
    let x: MyStruct<f32> = MyStruct { value: 0.0 };
    println!("x={}", x.value);
}
