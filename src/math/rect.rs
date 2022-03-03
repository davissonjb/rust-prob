/**
 * File:                 rect.rs
 * Author:            Jacob B Davisson
 * Purpose:         Encapsulate functionality for rectangular data types
 * Origination:     1 MARCH 2022
 * Modified:         1 MARCH 2022
 */

use std::ops::{Add, Sub, Mul, Div};

 pub struct rect<T>
 {
    x: T,
    y: T,
 }

 impl<T: Add<Output = T>> Add for rect<T>
 {
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
       Self {x: self.x + other.x, y: self.y + other.y}
    }
 }

 impl<T: Sub<Output=T>> Sub for rect<T>
 {
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
       Self {x: self.x - other.x, y: self.y - other.y}
    }
 }