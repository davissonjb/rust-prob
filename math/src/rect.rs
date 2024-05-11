//!
//! File:            prob-stats/math/src/rect.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     1 March 2024    
//! Modified:        10 May 2024  
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use crate::numtype::*;
use crate::polar::*;
use crate::scalar::*;
use num::rational::*;
use num::*;
use std::f32;
use std::f64;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub enum Quadrant {
    I,
    II,
    III,
    IV,
}

pub struct Rect<T: NumType> {
    a: Scalar<T>,
    b: Scalar<T>,
}

impl<T: NumType> Rect<T> {
    pub fn new() -> Self {
        Self {
            a: Scalar::<T>::new(),
            b: Scalar::<T>::new(),
        }
    }

    /// We will need to know which quadrant this rectangular
    /// form complex number is sitting in, for the purposes
    /// of identifying the correct tangent procedure to apply
    /// for correctly finding the polar angle.
    pub fn quad(&self) -> Quadrant {
        match (self.a >= Scalar::<T>::new(), self.b >= Scalar::<T>::new()) {
            (true, true) => Quadrant::I,
            (false, true) => Quadrant::II,
            (false, false) => Quadrant::III,
            (true, false) => Quadrant::IV,
        }
    }

    pub fn polar(&self) -> Polar<T> {
        Polar::<T>::new().setnew(
            (self.a.square() + self.b.square()).sqrt(),
            self.b.atan2(self.a),
        )
    }
}

/*
 * ---------------------------------------------------------------------------------
 * ---------------------------------------------------------------------------------
 * ----------------------  Utility trait implementations  --------------------------
 * ---------------------------------------------------------------------------------
 * ---------------------------------------------------------------------------------
 */

impl<T: NumType> Add for Rect<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl<T: NumType> Sub for Rect<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T: NumType> Mul for Rect<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a,
            b: self.b * other.b,
        }
    }
}

impl<T: NumType> Div for Rect<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            a: self.a / other.a,
            b: self.b / other.b,
        }
    }
}
