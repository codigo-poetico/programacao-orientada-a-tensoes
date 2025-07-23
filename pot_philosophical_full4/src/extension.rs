use crate::tension::Tension;

// Intensity vs Extensivity
pub trait IntensityExt {
    fn set_intensity(&mut self, val: f64);
}

pub trait ExtensivityExt {
    fn set_extensivity(&mut self, val: f64);
}

// Analogies
#[derive(Debug)]
pub struct Analogy {
    pub description: &'static str,
    func: Box<dyn Fn()>,
}

impl Analogy {
    pub fn new<F>(description: &'static str, f: F) -> Self
    where F: Fn() + 'static {
        Analogy { description, func: Box::new(f) }
    }
    pub fn apply(&self) { (self.func)(); }
}