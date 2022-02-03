use crate::math::types::{Complex, Polynomial};

pub fn newton(z: Complex, poly: Polynomial, deriv: Polynomial, num_iterations: usize) -> Complex {
    let mut z = z;

    for _ in 0..num_iterations {
        z = z + (poly.calc(z) / deriv.calc(z) * -1.0);
    }

    z
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

        let z = newton(
            Complex {
                real: 1.01,
                imag: 1.0,
            },
            poly,
            deriv,
            1000,
        );

        
        assert_eq!(z, 1.0.into());
    }
}
