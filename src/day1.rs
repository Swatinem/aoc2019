//! # Level1

/// Take its mass, divide by three, round down, and subtract 2.
/// Any mass that would require negative fuel should instead be treated as
/// if it requires zero fuel.
fn fuel_for_mass(mass: isize) -> isize {
    ((mass / 3) - 2).max(0)
}

/// For each module mass, calculate its fuel and add it to the total.
/// Then, treat the fuel amount you just calculated as the input mass and
/// repeat the process, continuing until a fuel requirement is zero or negative.
fn fuel_for_mass_recursive(mut mass: isize) -> isize {
    let mut total = 0;
    loop {
        mass = fuel_for_mass(mass);
        if mass > 0 {
            total += mass;
        } else {
            return total;
        }
    }
}

use crate::boilerplate::Input;

pub fn part1(input: &str) -> String {
    Input::new(input)
        .iter_num()
        .map(fuel_for_mass)
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    Input::new(input)
        .iter_num()
        .map(fuel_for_mass_recursive)
        .sum::<isize>()
        .to_string()
}
