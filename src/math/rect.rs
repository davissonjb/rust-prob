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
    a: T,
    b: T,
 }

 impl<T: Add<Output = T>> Add for rect<T>
 {
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
       Self {a: self.a + other.a, b: self.b + other.b}
    }
 }

 impl<T: Sub<Output=T>> Sub for rect<T>
 {
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
       Self {a: self.a - other.a, b: self.b - other.b}
    }
 }