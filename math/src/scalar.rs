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

use num::rational::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

pub struct Scalar<T: num::Float + Default + Display>(T);

impl<T: num::Float + Default + Display> Scalar<T> {
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
}

impl<T: num::Float + Default + Display> Display for Scalar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ------------------------ std::ops::Add Implementations ---------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
impl<T: num::Float + Default + Display> Add for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 + rhs.0)
    }
}

impl<T: num::Float + Default + Display> Add<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 + T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Add<i64> for Scalar<T> {
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
impl<T: num::Float + Default + Display> Mul for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 * rhs.0)
    }
}

impl<T: num::Float + Default + Display> Mul<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 * T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Mul<i64> for Scalar<T> {
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
impl<T: num::Float + Default + Display> Sub for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 - rhs.0)
    }
}

impl<T: num::Float + Default + Display> Sub<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 - T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Sub<i64> for Scalar<T> {
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
impl<T: num::Float + Default + Display> Div for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Scalar::<T>::setnew(self.0 / rhs.0)
    }
}

impl<T: num::Float + Default + Display> Div<f32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: f32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<f64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: f64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<u8> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u8) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<u16> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u16) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<u32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<u64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: u64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<i8> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i8) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<i16> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i16) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<i32> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i32) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}

impl<T: num::Float + Default + Display> Div<i64> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: i64) -> Self::Output {
        Scalar::<T>::setnew(self.0 / T::from(rhs).unwrap())
    }
}
