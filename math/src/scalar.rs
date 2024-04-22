//!
//! File:            prob-stats/math/src/scalar.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     23 February 2024
//! Modified:        22 April 2024
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use num::rational::*;
use std::fmt::{Debug, Display};

pub struct Scalar<T: num::Float + Default>(T);

impl<T: num::Float + Default> Scalar<T> {
    pub fn new() -> Self {
        Self(T::Default)
    }
}
