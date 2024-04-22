//!
//! File:            prob-stats/err/src/lib.rs
//! Author:          Jacob B Davisson
//! Purpose:         Encapsulate functionality for rectangular data types
//! Origination:     23 February 2024
//! Modified:        22 April 2024
//!

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
