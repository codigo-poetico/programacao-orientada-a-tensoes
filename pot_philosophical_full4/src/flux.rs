use crate::schema::Schema;
#[derive(Debug)]
pub struct Flux {
    pub history: Vec<Schema>,
}

impl Flux {
    pub fn new() -> Self { Flux { history: Vec::new() } }
    pub fn record(&mut self, state: &Schema) {
        self.history.push(state.clone());
    }
}