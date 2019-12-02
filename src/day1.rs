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

#[test]
fn test_part1() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);
}

#[test]
fn test_part2() {
    assert_eq!(fuel_for_mass_recursive(14), 2);
    assert_eq!(fuel_for_mass_recursive(1969), 966);
    assert_eq!(fuel_for_mass_recursive(100756), 50346);
}

fn iter_num<'a>(input: &'a str) -> impl Iterator<Item = isize> + 'a {
    input
        .split_whitespace()
        .map(|t| t.parse().expect("parsing number"))
}

pub fn part1(input: &str) -> String {
    iter_num(input)
        .map(fuel_for_mass)
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    iter_num(input)
        .map(fuel_for_mass_recursive)
        .sum::<isize>()
        .to_string()
}
