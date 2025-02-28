#[derive(Clone, Debug)]
pub(crate) struct Value(pub(crate) f32);

impl Value {
    pub(crate) fn print(&self) {
        print!("{:?}", self.0);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Values {
    pub(crate) values: Vec<Value>,
}

impl Values {
    pub(crate) fn print_value(&self, index: usize) {
        self.values
            .get(index)
            .expect(&format!("Failed to get constant at index {}", index))
            .print()
    }
}
