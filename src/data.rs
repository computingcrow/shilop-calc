use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use crate::data::Data::{Complex, Double};

pub enum Data {
    Double(f64),
    Complex(f64, f64)
}

impl Data {
    pub fn real(&self) -> f64 {
        match self {
            Double(d) => *d,
            Complex(r, _) => *r
        }
    }

    pub fn im(&self) -> Option<f64> {
        match self {
            Double(_) => None,
            Complex(_, c) => Some(*c)
        }
    }

    pub fn abs(&self) -> Data {
        Double(self.abs_as_f64())
    }

    pub fn abs_as_f64(&self) -> f64 {
        match self {
            Double(d) => d.abs(),
            Complex(r, c) => (r.powf(2f64) + c.powf(2f64)).sqrt()
        }
    }

    pub fn pow(&self, rhs: Data) -> Data {
        match self {
            Double(sd) => {
                match rhs {
                    Double(od) => Double(sd.powf(od)),
                    Complex(or, _) => Double(sd.powf(or))
                }
            },
            Complex(sr, _) => {
                match rhs {
                    Double(od) => Double(sr.powf(od)),
                    Complex(or, _) => Double(sr.powf(or))
                }
            }
        }
    }

    pub fn exp(&self) -> Data {
        match self {
            Double(d) => {
                Double(d.exp())
            },
            Complex(sr, sc) => {
                Complex(
                    sr.exp() * sc.cos(),
                    sr.exp() * sc.sin()
                )
            }
        }
    }

    pub fn sin(&self) -> Option<Data> {
        match self {
            Double(d) => Some(Double(d.sin())),
            _ => None
        }
    }

    pub fn cos(&self) -> Option<Data> {
        match self {
            Double(d) => Some(Double(d.cos())),
            _ => None
        }
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Double(sd) => {
                match rhs {
                    Double(od) => {
                        Double(sd + od)
                    }
                    Complex(or, oc) => {
                        Complex(sd + or, oc)
                    }
                }
            }
            Complex(sr, sc) => {
                match rhs {
                    Double(od) => {
                        Complex(sr + od, sc)
                    }
                    Complex(or, oc) => {
                        Complex(sr + or, sc + oc)
                    }
                }
            }
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Double(sd) => {
                match rhs {
                    Double(od) => Double(sd - od),
                    Complex(or, oc) => Complex(sd - or, oc)
                }
            }
            Complex(sr, sc) => {
                match rhs {
                    Double(od) => Complex(sr - od, sc),
                    Complex(or, oc) => Complex(sr - or, sc - oc)
                }
            }
        }
    }
}

impl Mul for Data {
    type Output = Data;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Double(sd) => {
                match rhs {
                    Double(od) => Double(sd * od),
                    Complex(or, oc) => Complex(sd * or, oc)
                }
            }
            Complex(sr, sc) => {
                match rhs {
                    Double(od) => Complex(sr * od, sc),
                    Complex(or, oc) => Complex(sr * or - sc * oc, sr * oc + sc * or)
                }
            }
        }
    }
}

fn complex_divide(x: Data, y: Data) -> Data {
    match x {
        Complex(sr, sc) => {
            match y {
                Complex(or, oc) => Complex(sr * or - sc * oc, sr * oc + sc * or),
                _ => panic!("Invalid use of complex divide")
            }
        }
        _ => panic!("Invalid use of complex divide")
    }
}

impl Div for Data {
    type Output = Data;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Double(sd) => {
                match rhs {
                    Double(od) => Double(sd / od),
                    _ => complex_divide(Complex(sd, 0f64), rhs)
                }
            }
            _ => {
                match rhs {
                    Double(od) => complex_divide(self, Complex(od, 0f64)),
                    _ => complex_divide(self, rhs)
                }
            }
        }
    }
}

impl Sum for Data {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let sum = iter.fold(Complex(0f64, 0f64), |l, r| l + r);
        if sum.im().unwrap() != 0f64 {
            return sum
        } else {
            Double(sum.real())
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        match self {
            Double(d) => Double(*d),
            Complex(r, c) => Complex(*r, *c)
        }
    }
}

impl Copy for Data {

}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Double(d) => write!(f, "{}", d),
            Data::Complex(r, c) => write!(f, "{}+{}i", r, c),
        }
    }
}
