use std::fmt;
use std::ops::{Add, Div, Mul};

use crate::types::Complex;

impl Complex {
    pub fn pow(self, n: i32) -> Complex {
        let mut res = self;
        for _i in 1..n {
            res = res * self;
        }
        res
    }
}

impl From<f64> for Complex {
    fn from(n: f64) -> Complex {
        Complex { real: n, imag: 0.0 }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: (self.real * other.real) - (self.imag * other.imag),
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let sum_square = other.real.powi(2) + other.imag.powi(2);
        Complex {
            real: (self.real * other.real + self.imag * other.imag) / sum_square,
            imag: (self.imag * other.real - self.real * other.imag) / sum_square,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, other: f64) -> Complex {
        Complex {
            real: self.real * other,
            imag: self.imag * other,
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        (self.real - other.real).abs() < 1e-15 && (self.imag - other.imag).abs() < 1e-15
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag < f64::EPSILON {
            write!(f, "{}", self.real)
        } else if self.imag < 0.0 {
            write!(f, "{}{}i", self.real, self.imag)
        } else {
            write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_complexes() {
        assert_eq!(
            Complex {
                real: 1.0,
                imag: 1.0
            } + Complex {
                real: 0.1,
                imag: -0.1
            },
            Complex {
                real: 1.1,
                imag: 0.9
            }
        );
    }

    #[test]
    fn it_powers_complexes() {
        assert_eq!(
            Complex {
                real: 0.0,
                imag: 1.0
            }
            .pow(2),
            Complex {
                real: -1.0,
                imag: 0.0
            }
        );

        assert_eq!(
            Complex {
                real: -1.0,
                imag: 0.0
            }
            .pow(2),
            Complex {
                real: 1.0,
                imag: 0.0
            }
        );
    }

    #[test]
    fn it_muls_integers() {
        assert_eq!(
            Complex {
                real: 1.0,
                imag: 0.1
            } * 0.1,
            Complex {
                real: 0.1,
                imag: 0.01
            }
        )
    }

    #[test]
    fn it_muls_complexes() {
        assert_eq!(
            Complex {
                real: 3.0,
                imag: 2.0
            } * Complex {
                real: 1.0,
                imag: 4.0
            },
            Complex {
                real: -5.0,
                imag: 14.0
            }
        )
    }
}
