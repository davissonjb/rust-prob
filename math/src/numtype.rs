//!
//! File:            prob-stats/math/src/mathtype.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     23 February 2024
//! Modified:        10 May 2024     
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use num::rational::*;
use std::fmt::Display;

pub trait NumType: num::Float + Default + Display {}

impl NumType for f32 {}
impl NumType for f64 {}
