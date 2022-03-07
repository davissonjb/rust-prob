/**
 * File:                 polar.rs
 * Author:            Jacob B Davisson
 * Purpose:         Encapsulate functionality for polar data types
 * Origination:     1 MARCH 2022
 * Modified:         1 MARCH 2022
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
    pub fn conj(&self) -> Self
    {
        Self {mag: self.mag, ang: self.ang}
    }

    pub fn Mag(&self) -> T
    {
        self.mag
    }

    pub fn Ang(&self) -> T
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