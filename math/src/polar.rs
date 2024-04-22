//!
//! File:            prob-stats/math/src/polar.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     1 March 2022     
//! Modified:        22 April 2024   
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

//
//
//
//
pub struct Polar<T> {
    pub mag: T,
    pub ang: T,
}

impl<T: Sub + Copy> Polar<T> {
    ///
    /// Function: pub fn conj(&self) -> Self
    /// Purpose: Apply conjugation to complex number
    ///          in polar form.
    ///
    pub fn conj(&self) -> Self {
        Self {
            mag: self.mag,
            ang: self.ang,
        }
    }

    ///
    /// Function: pub fn mag(&self) -> Self
    /// Purpose: Return magnitude of Polar form
    ///          complex number.
    ///
    pub fn mag(&self) -> T {
        self.mag
    }

    ///
    /// Function: pub fn ang(&self) -> T
    /// Purpose: Return angle of Polar form
    ///          complex number.
    ///
    pub fn ang(&self) -> T {
        self.ang
    }
}

/*impl<T: Add<Output = T>> Add for Polar<T>
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        Self {}
    }
}*/
