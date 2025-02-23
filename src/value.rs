#[derive(Debug)]
pub(crate) struct Value(pub(crate) f32);

pub(crate) struct Values {
    pub(crate) values: Vec<Value>,
}

impl Values {
    pub(crate) fn init() -> Self {
        Self { values: Vec::new() }
    }

    pub(crate) fn print_value(&self, index: &usize) {
        print!(
            "{:?}",
            self.values
                .get(index.clone())
                .expect(format!("Failed to get constant at index {}", index).as_str())
        )
    }
}
