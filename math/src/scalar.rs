#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use num::rational::*;
use std::fmt::{Debug, Display};

/**
 * File:            /math/src/scalar.rs
 * Author:          Jacob B Davisson
 * Purpose:         Encapsulate functionality for scalar data types
 * Origination:     23 Februaty 2024
 * Modified:        23 Februaty 2024
 */

pub struct Scalar<T: num::Float + Default>(T);

impl<T: num::Float + Default> Scalar<T> {
    pub fn new() -> Self {
        Self(T::Default)
    }
}
