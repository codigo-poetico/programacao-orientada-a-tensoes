#[derive(Debug, Clone)]
pub struct MaiorDeIdade(pub u8);

impl MaiorDeIdade {
    pub const fn new(val: u8) -> Option<Self> {
        if val >= 18 {
            Some(MaiorDeIdade(val))
        } else {
            None
        }
    }
}