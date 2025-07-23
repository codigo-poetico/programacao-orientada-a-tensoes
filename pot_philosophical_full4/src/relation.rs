#[derive(Debug, Clone)]
pub struct Relation {
    pub description: &'static str,
    func: Box<dyn Fn() -> bool>,
}

impl Relation {
    pub fn new<F>(description: &'static str, f: F) -> Self
    where F: Fn() -> bool + 'static {
        Relation { description, func: Box::new(f) }
    }
    pub fn check(&self) -> bool { (self.func)() }
}