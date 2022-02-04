use crate::math::types::{Complex, Polynomial};

pub fn newton(z: Complex, poly: &Polynomial, deriv: &Polynomial, num_iterations: usize) -> Complex {
    let mut z = z;

    for _ in 0..num_iterations {
        z = z + ((poly.calc(z) / deriv.calc(z)) * -1.0);
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

        assert_eq!(
            newton(
                Complex {
                    real: 1.01,
                    imag: 1.0,
                },
                &poly,
                &deriv,
                1000,
            ),
            1.0.into()
        );

        assert_eq!(
            newton(
                Complex {
                    real: -10.6,
                    imag: -5.96,
                },
                &poly,
                &deriv,
                10000,
            ),
            Complex {
                real: -0.5,
                imag: -0.8660254037844386467637,
            }
        );
    }
}
