//!
//! File:            prob-stats/math/src/polar.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     1 March 2022     
//! Modified:        10 May 2024     
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use crate::numtype::*;
use crate::rect::*;
use crate::scalar::*;
use std::ops::{Add, Div, Mul, Sub};

//
//
//
//
pub struct Polar<T: NumType> {
    mag: Scalar<T>,
    ang: Scalar<T>,
}

impl<T: NumType> Polar<T> {
    pub fn new() -> Self {
        Polar {
            mag: Scalar::<T>::new(),
            ang: Scalar::<T>::new(),
        }
    }

    pub fn set(&mut self, m: Scalar<T>, a: Scalar<T>) -> () {
        self.mag = m;
        self.ang = a;
    }

    pub fn setnew(&self, m: Scalar<T>, a: Scalar<T>) -> Polar<T> {
        Polar { mag: m, ang: a }
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
