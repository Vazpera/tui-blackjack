use cgmath::{vec3, Vector3};
pub mod func;

fn main() {
    let line_1: Vector3<i32> = vec3(1, 2, 3);

    println!("line_1.x    : {}", line_1.x);
    println!("line_1.y    : {}", line_1.y);
    println!("line_1.z    : {}", line_1.z);
}
