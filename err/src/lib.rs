//!
//! File:            prob-stats/err/src/lib.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     23 February 2024
//! Modified:        13 May 2024  
//!

use std::error::Error;
use std::fmt::{Debug, Display};

pub enum AlgebraError {
    DivisionByZero,
    IncompatibleTypes,
}

impl Debug for AlgebraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgebraError::DivisionByZero => {
                write!(f, "[AlgErr]:: {}:{} -- Division by zero", file!(), line!())
            }
            AlgebraError::IncompatibleTypes => {
                write!(
                    f,
                    "[AlgErr]:: {}:{} -- Incompatible types for operation",
                    file!(),
                    line!()
                )
            }
        }
    }
}

impl Display for AlgebraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgebraError::DivisionByZero => write!(f, "[AlgErr]:: Division by zero"),
            AlgebraError::IncompatibleTypes => {
                write!(f, "[AlgErr]:: Incompatible types for operation")
            }
        }
    }
}

impl Error for AlgebraError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
        // match self {
        //     AlgebraError::DivisionByZero => Some(AlgebraError::DivisionByZero),
        //     AlgebraError::IncompatibleTypes => Some(AlgebraError::IncompatibleTypes),
        // }
    }
}

//
//
//
//
//
//
//
//
//
