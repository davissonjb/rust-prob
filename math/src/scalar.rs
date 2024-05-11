//!
//! File:            prob-stats/math/src/scalar.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     23 February 2024
//! Modified:        10 May 2024  
//!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use crate::numtype::*;
use num::rational::*;
use num::Float;
use num_traits::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Scalar<T: NumType>(T);

impl<T: NumType> Scalar<T> {
    pub fn new() -> Self {
        Self(T::default())
    }

    pub fn set(&mut self, s: T) -> () {
        self.0 = s;
    }

    pub fn setnew(s: T) -> Self {
        Self(s)
    }

    pub fn val(&self) -> T {
        self.0
    }

    pub fn atan2(&self, rhs: Self) -> Scalar<T> {
        Scalar::<T>::setnew(self.val().atan2(rhs.val()))
    }

    pub fn square(&self) -> Scalar<T> {
        Scalar::<T>::setnew(self.val() * self.val())
    }

    pub fn sqrt(&self) -> Scalar<T> {
        Scalar::<T>::setnew(self.val().sqrt())
    }
}

impl<T: NumType> Display for Scalar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ------------------------ std::ops::Add Implementations ---------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
impl<T: NumType> Add for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 + rhs.0)
    }
}

impl<T: NumType> Add<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: NumType> Add<i64> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i64) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ------------------------ std::ops::Mul Implementations ---------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
//
impl<T: NumType> Mul for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 * rhs.0)
    }
}

impl<T: NumType> Mul<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: NumType> Mul<i64> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i64) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ------------------------ std::ops::Sub Implementations ---------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
//
impl<T: NumType> Sub for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 - rhs.0)
    }
}

impl<T: NumType> Sub<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: NumType> Sub<i64> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i64) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ------------------------ std::ops::Div Implementations ---------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
//
impl<T: NumType> Div for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 / rhs.0)
    }
}

impl<T: NumType> Div<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: NumType> Div<i64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}
