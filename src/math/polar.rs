#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]
/**
 * File:            polar.rs
 * Author:          Jacob B Davisson
 * Purpose:         Encapsulate functionality for polar data types
 * Origination:     1 MARCH 2022
 * Modified:        26 JUNE 2023
 */

use std::ops::{Add, Sub, Mul, Div};

//
//
//
//
pub struct Polar<T>
{
    pub mag: T,
    pub ang: T,
}

impl<T: Sub + Copy> Polar<T>
{
    ///
    /// Function: pub fn conj(&self) -> Self 
    /// Purpose: Apply conjugation to complex number
    ///          in polar form.
    /// 
    pub fn conj(&self) -> Self
    {
        Self {mag: self.mag, ang: self.ang}
    }

    ///
    /// Function: pub fn mag(&self) -> Self 
    /// Purpose: Return magnitude of Polar form 
    ///          complex number.
    /// 
    pub fn mag(&self) -> T
    {
        self.mag
    }

    ///
    /// Function: pub fn ang(&self) -> T
    /// Purpose: Return angle of Polar form
    ///          complex number.
    ///
    pub fn ang(&self) -> T
    {
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