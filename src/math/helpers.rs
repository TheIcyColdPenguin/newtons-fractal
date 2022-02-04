use crate::math::types::{Complex, Polynomial};

pub struct NewtonIterator<'a> {
    z: Complex,
    poly: &'a Polynomial,
    deriv: &'a Polynomial,
    n: usize,
    max_iterations: usize,
}
impl<'a> NewtonIterator<'a> {
    pub fn new(
        z: Complex,
        poly: &'a Polynomial,
        deriv: &'a Polynomial,
        max_iterations: usize,
    ) -> NewtonIterator<'a> {
        NewtonIterator {
            z,
            poly,
            deriv,
            max_iterations,
            n: 0,
        }
    }

    pub fn next(&mut self) -> Option<Complex> {
        if self.n == 0 {
            self.n += 1;
            Some(self.z)
        } else if self.n >= self.max_iterations {
            None
        } else {
            self.n += 1;
            self.z = self.z + ((self.poly.calc(self.z) / self.deriv.calc(self.z)) * -1.0);

            Some(self.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::types::Item;

    #[test]
    fn it_calculates_convergent_point() {
        let poly = Polynomial(vec![
            Item {
                pow: 3,
                coeff: 1.0.into(),
            },
            Item {
                pow: 0,
                coeff: (-1.0).into(),
            },
        ]);

        let deriv = poly.derivative();

        let mut newton1 = NewtonIterator::new(
            Complex {
                real: 1.01,
                imag: 1.0,
            },
            &poly,
            &deriv,
            1000,
        );

        let mut newton2 = NewtonIterator::new(
            Complex {
                real: -10.6,
                imag: -5.96,
            },
            &poly,
            &deriv,
            10000,
        );

        for _ in 0..999 {
            newton1.next();
            newton2.next();
        }

        assert_eq!(newton1.next(), Some(1.0.into()));

        assert_eq!(
            newton2.next(),
            Some(Complex {
                real: -0.5,
                imag: -0.8660254037844386467637,
            })
        );
    }
}
