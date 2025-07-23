use std::collections::HashMap;
use crate::tension::Tension;
use crate::relation::Relation;
use crate::corruption::Corruption;

#[derive(Debug, Clone)]
pub struct Schema {
    tensions: HashMap<&'static str, Box<dyn AnyTension>>,
    relations: Vec<Relation>,
}

impl Schema {
    pub fn new() -> Self {
        Self { tensions: HashMap::new(), relations: Vec::new() }
    }
    pub fn add_tension<T: 'static + std::fmt::Display + Clone>(
        &mut self,
        mut tension: Tension<T>,
    ) {
        self.tensions.insert(tension.name, Box::new(tension));
    }
    pub fn add_relation(&mut self, r: Relation) { self.relations.push(r); }
    pub fn set<T: 'static + std::fmt::Display>(&mut self, name: &'static str, val: T) {
        if let Some(t) = self.tensions.get_mut(name) { t.set_boxed(val); }
    }
    pub fn is_corrupted(&self) -> bool {
        self.tensions.values().any(|t| t.corruption().is_some()) ||
        self.relations.iter().any(|r| !r.check())
    }
    pub fn ensure_concrete(&self) -> bool {
        self.tensions.values().all(|t| t.has_value())
    }
    pub fn apply_homeostasis(&mut self) {
        // Iterate tensions and re-set values to apply homeostasis if applicable
        for (_, t) in self.tensions.iter_mut() {
            if let Some(val) = t.get_value() {
                t.set_boxed(val.clone());
            }
        }
    }
}

pub trait AnyTension: std::fmt::Debug {
    fn set_boxed(&mut self, val: impl std::fmt::Display + 'static);
    fn corruption(&self) -> Option<&Corruption>;
    fn has_value(&self) -> bool;
    fn get_value(&self) -> Option<Box<dyn std::fmt::Display>>;
}

impl<T: 'static + std::fmt::Display + Clone> AnyTension for Tension<T> {
    fn set_boxed(&mut self, val: impl std::fmt::Display + 'static) {
        let v: T = val.to_string().parse().ok().unwrap();
        self.set(v);
    }
    fn corruption(&self) -> Option<&Corruption> { self.corruption.as_ref() }
    fn has_value(&self) -> bool { self.value.is_some() }
    fn get_value(&self) -> Option<Box<dyn std::fmt::Display>> {
        self.value.as_ref().map(|v| Box::new(v.clone()) as Box<dyn std::fmt::Display>)
    }
}