//!
//! File:            prob-stats/math/src/rect.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     1 March 2024    
//! Modified:        22 April 2024
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use crate::scalar::*;
use num::rational::*;
use std::ops::{Add, Div, Mul, Sub};

// pub struct Rect<T: num::Float + Default> {
//     a: Scalar<T>,
//     b: Scalar<T>,
// }

/*
 * ---------------------------------------------------------------------------------
 * ---------------------------------------------------------------------------------
 * ----------------------  Utility trait implementations  --------------------------
 * ---------------------------------------------------------------------------------
 * ---------------------------------------------------------------------------------
 */

// impl<T: Add<Output = T>> Add for Rect<T> {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Self {
//             a: self.a + other.a,
//             b: self.b + other.b,
//         }
//     }
// }

// impl<T: Sub<Output = T>> Sub for Rect<T> {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         Self {
//             a: self.a - other.a,
//             b: self.b - other.b,
//         }
//     }
// }
//
