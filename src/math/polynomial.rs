use std::fmt;
use std::ops::{Add, Neg};

use crate::math::types::Complex;
use crate::math::types::{Item, Polynomial};

impl Polynomial {
    pub fn calc(&self, x: Complex) -> Complex {
        self.0
            .iter()
            .fold(0.0.into(), |acc, item| acc + item.coeff * x.pow(item.pow))
    }

    pub fn derivative(&self) -> Polynomial {
        let mut new_poly: Polynomial = vec![].into();

        for i in self.0.iter() {
            if i.pow == 0 {
                continue;
            }
            new_poly.0.push(Item {
                coeff: i.coeff * i.pow as f64,
                pow: i.pow - 1,
            })
        }

        new_poly
    }
}

impl From<Vec<f64>> for Polynomial {
    fn from(vector: Vec<f64>) -> Polynomial {
        Polynomial(
            vector
                .into_iter()
                .enumerate()
                .map(|(i, item)| Item {
                    pow: i as i32,
                    coeff: item.into(),
                })
                .collect(),
        )
    }
}

impl From<&Item> for String {
    fn from(item: &Item) -> String {
        if item.pow == 0 {
            format!("{:?}", item.coeff)
        } else if item.pow == 1 {
            format!("({:?}z)", item.coeff)
        } else {
            format!("({:?}z^{})", item.coeff, item.pow)
        }
    }
}

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P(z) = {}",
            self.0
                .iter()
                .map(|i| String::from(i))
                .collect::<Vec<String>>()
                .join("+")
        )
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Polynomial {
        let mut poly = Polynomial(vec![]);

        for item in self.0.iter() {
            poly.0.push(Item {
                coeff: item.coeff * -1.0,
                pow: item.pow,
            });
        }
        poly
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        let mut poly = Polynomial(vec![]);

        for i in self.0.iter() {
            poly.0.push(Item {
                pow: i.pow,
                coeff: if let Some(item) = other.0.iter().find(|each_item| each_item.pow == i.pow) {
                    i.coeff + item.coeff
                } else {
                    i.coeff
                },
            });
        }
        for i in other.0.iter() {
            if self
                .0
                .iter()
                .find(|each_item| each_item.pow == i.pow)
                .is_none()
            {
                poly.0.push(Item {
                    pow: i.pow,
                    coeff: i.coeff,
                });
            }
        }

        poly
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Polynomial) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |acc, (item1, item2)| acc && (item1 == item2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_negates_polynomials() {
        assert_eq!(-Polynomial::from(vec![0.1]), Polynomial::from(vec![-0.1]));
        assert_eq!(
            -Polynomial::from(vec![0.1, 1.4, 3.1415]),
            Polynomial::from(vec![-0.1, -1.4, -3.1415])
        );
    }

    #[test]
    fn it_adds_polynomials() {
        assert_eq!(
            Polynomial::from(vec![1.1]) + Polynomial::from(vec![0.1]),
            Polynomial::from(vec![1.2])
        );
        assert_eq!(
            -Polynomial::from(vec![0.1]) + Polynomial::from(vec![0.1]),
            Polynomial::from(vec![0.0])
        );
        assert_eq!(
            Polynomial(vec![
                Item {
                    pow: 0,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 1,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 3,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 5,
                    coeff: 1.0.into(),
                },
            ]) + Polynomial(vec![
                Item {
                    pow: 0,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 2,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 4,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 6,
                    coeff: 1.0.into(),
                },
            ]),
            Polynomial(vec![
                Item {
                    pow: 0,
                    coeff: 2.0.into(),
                },
                Item {
                    pow: 1,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 3,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 5,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 2,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 4,
                    coeff: 1.0.into(),
                },
                Item {
                    pow: 6,
                    coeff: 1.0.into(),
                },
            ])
        )
    }

    #[test]
    fn it_calcs_derivative() {
        assert_eq!(
            Polynomial(vec![
                Item {
                    pow: 0,
                    coeff: 2.0.into()
                },
                Item {
                    pow: 1,
                    coeff: 3.0.into()
                },
                Item {
                    pow: 2,
                    coeff: 4.0.into()
                },
            ])
            .derivative(),
            Polynomial(vec![
                Item {
                    pow: 0,
                    coeff: 3.0.into()
                },
                Item {
                    pow: 1,
                    coeff: 8.0.into()
                },
            ])
        );
    }

    #[test]
    fn it_calcs_value() {
        assert_eq!(
            Polynomial(vec![Item {
                pow: 1,
                coeff: 2.0.into()
            }])
            .calc(Complex {
                real: 0.1,
                imag: 1.1
            }),
            Complex {
                real: 0.2,
                imag: 2.2
            }
        );
    }
}
