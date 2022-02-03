#[derive(PartialEq)]
pub struct Item {
    pub pow: i32,
    pub coeff: Complex,
}

pub struct Polynomial(pub Vec<Item>);

#[derive(Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}
