pub struct Value {
    pub value: i32,
}

impl Default for Value {
    fn default() -> Value {
        Self { value: 0 }
    }
}

impl Value {
    pub fn value(self, val: i32) -> Self {
        Self {
            value: val
        }
    }
}
