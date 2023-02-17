
use std::ops::{Add, Mul, Sub};
use std::{fmt, fmt::Display};

#[derive(Debug)]
pub struct Species {
    formula: String,
    cc: f64,
}

impl Species {
    pub fn new(formula:String) -> Self {
        Self {formula, cc:0 as f64}
    }

    pub fn cc(&self) -> f64 { self.cc }
    pub fn set_cc(&mut self, cc:f64) { self.cc = cc; }
}


// For use in println!()
impl Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.formula)
    }
}


// Implements Basic maths operations between Species.
// --> Act on cc
impl<'a, 'b> Add<&'b Species> for &'a Species {
    type Output = f64;
    fn add(self, other:&'b Species) -> f64 {
        self.cc + other.cc
    }
}

impl<'a, 'b> Mul<&'b Species> for &'a Species {
    type Output = f64;
    fn mul(self, other:&'b Species) -> f64 {
        self.cc * other.cc
    }
}

impl<'a, 'b> Sub<&'b Species> for &'a Species {
    type Output = f64;
    fn sub(self, other:&'b Species) -> f64 {
        let res = self.cc - other.cc;
        // Calculated cc cannot be negative.
        if res <= 0.0 {
            return 0 as f64;
        } else {
            return res;
        }

    }
}