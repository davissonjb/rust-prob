/**
 * File:                 main.rs
 * Author:            Jacob B Davisson
 * Purpose:         This is where the program testing occurs...should likely be converted
 *                              to a library project, but will do so later.
 * Origination:     1 MARCH 2022
 * Modified:         1 MARCH 2022
 */
mod math;
use math::polar::Polar;

fn main() {
    let a: Polar::<f64> = Polar::<f64> {mag: 2.0, ang: 30.0};
    println!("Magnitude: {} -- Angle: {}", a.Mag(), a.Ang());
    println!("Hello, world!");
}
