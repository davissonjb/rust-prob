/**
 * File:                 rect.rs
 * Author:            Jacob B Davisson
 * Purpose:         Encapsulate functionality for rectangular data types
 * Origination:     1 MARCH 2022
 * Modified:         1 MARCH 2022
 */

use std::ops::{Add, Sub, Mul, Div}

 pub struct rect<T>
 {
    x: T,
    y: T,
 }

 impl<T> Add for rect<T>
 {
    type Output = Self;
    fn Add(self, other: Self) -> Self
    {
       Self {s: self.x + other.x, y: self.y + other.y}
    }
 }