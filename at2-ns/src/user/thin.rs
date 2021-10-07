#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThinUser {
    name: String,
}

impl ThinUser {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
