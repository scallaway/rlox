#[derive(Debug)]
pub struct Value(pub f64);

impl Value {
    pub fn print(value: &Value) {
        print!("{:?}", value.0);
    }
}

// NOTE: Have a think about whether we need this in Rust, since it's just a
//       wrapper for a `Vec<Value>`.
//       If it's going to contain more, then we should definitely keep it.
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn init() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn free(&self) -> ValueArray {
        ValueArray::init()
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}
