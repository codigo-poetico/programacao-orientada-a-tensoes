#[derive(Debug, Clone)]
pub struct Corruption {
    pub tension: &'static str,
    pub received: String,
    pub message: &'static str,
}

impl Corruption {
    pub fn new(tension: &'static str, received: String, message: &'static str) -> Self {
        Self { tension, received, message }
    }
}