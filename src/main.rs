#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]
/**
 * File:            main.rs
 * Author:          Jacob B Davisson
 * Purpose:         This is where the program testing occurs...should likely be converted
 *                  to a library project, but will do so later.
 * Origination:     1 MARCH 2022
 * Modified:        26 JUNE 2023
 */


mod math;
use math::polar::Polar;

fn main() {
    let a: Polar::<f64> = Polar::<f64> {mag: 2.0, ang: 30.0};
    println!("Magnitude: {} -- Angle: {}", a.mag(), a.ang());
    println!("Hello, world!");
}
