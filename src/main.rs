pub mod func;

fn main() {
    println!(
        "Default {}",
        func::Value::default().set_value(2).get_value()
    );
}
