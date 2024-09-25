pub struct Value {
    pub value: i32,
}

impl Default for Value {
    fn default() -> Value {
        Self { value: 0 }
    }
}

impl Value {
    pub fn set_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }

    pub fn get_value(&self)-> i32 {
        self.value
    }
}
