#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]
/**
 * File:            rect.rs
 * Author:          Jacob B Davisson
 * Purpose:         Encapsulate functionality for rectangular data types
 * Origination:     1 MARCH 2022
 * Modified:        26 JUNE 2023
 */

use std::ops::{Add, Sub, Mul, Div};

pub struct Rect<T>
{
   a: T,
   b: T,
}

/*
 * ---------------------------------------------------------------------------------
 * ---------------------------------------------------------------------------------
 * ----------------------  Utility trait implementations  --------------------------
 * ---------------------------------------------------------------------------------
 * --------------------------------------------------------------------------------- 
 */

impl<T: Add<Output = T>> Add for Rect<T>
{
   type Output = Self;
   fn add(self, other: Self) -> Self
   {
      Self {a: self.a + other.a, b: self.b + other.b}
   }
}


impl<T: Sub<Output=T>> Sub for Rect<T>
{
   type Output = Self;
   fn sub(self, other: Self) -> Self
   {
      Self {a: self.a - other.a, b: self.b - other.b}
   }
}