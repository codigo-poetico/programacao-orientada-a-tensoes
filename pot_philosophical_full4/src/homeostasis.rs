pub trait Homeostasis<T> {
    fn attempt_recovery(&self, val: &T) -> Option<T>;
}

pub struct DynHomeostasis<T> {
    pub name: &'static str,
    func: Box<dyn Fn(&T) -> Option<T>>,
}

impl<T> DynHomeostasis<T> {
    pub fn new<F>(name: &'static str, f: F) -> Self
    where F: Fn(&T) -> Option<T> + 'static {
        Self { name, func: Box::new(f) }
    }
}

impl<T> Homeostasis<T> for DynHomeostasis<T>
where T: Clone {
    fn attempt_recovery(&self, val: &T) -> Option<T> {
        (self.func)(val)
    }
}