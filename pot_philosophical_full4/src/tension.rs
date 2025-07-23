use crate::homeostasis::Homeostasis;
use crate::corruption::Corruption;
#[derive(Debug, Clone)]
pub struct Tension<T> {
    pub name: &'static str,
    rule: Box<dyn Fn(&T) -> bool>,
    msg: &'static str,
    pub value: Option<T>,
    pub corruption: Option<Corruption>,
    homeo: Option<Box<dyn Homeostasis<T>>>,
    pub intensity: Option<f64>,
    pub extensivity: Option<f64>,
}

impl<T> Tension<T> {
    pub fn new<F>(name: &'static str, rule: F, msg: &'static str) -> Self
    where F: Fn(&T) -> bool + 'static {
        Self {
            name,
            rule: Box::new(rule),
            msg,
            value: None,
            corruption: None,
            homeo: None,
            intensity: None,
            extensivity: None,
        }
    }
    pub fn with_homeostasis<H>(mut self, h: H) -> Self
    where H: Homeostasis<T> + 'static {
        self.homeo = Some(Box::new(h));
        self
    }
    pub fn set(&mut self, val: T) {
        if (self.rule)(&val) {
            self.value = Some(val);
            self.corruption = None;
        } else if let Some(ref h) = self.homeo {
            if let Some(adj) = h.attempt_recovery(&val) {
                if (self.rule)(&adj) {
                    self.value = Some(adj);
                    self.corruption = None;
                    return;
                }
            }
            self.corruption = Some(Corruption::new(self.name, format!("{:?}", val), self.msg));
        } else {
            self.corruption = Some(Corruption::new(self.name, format!("{:?}", val), self.msg));
        }
    }
    pub fn set_intensity(&mut self, val: f64) { self.intensity = Some(val); }
    pub fn set_extensivity(&mut self, val: f64) { self.extensivity = Some(val); }
}