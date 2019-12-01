/// # Some Boilerplate
/// * An input tokenizer
/// * Directly testing a level on some fixtures
/// * Processing a level from an external input file
mod boilerplate {
    use std::fs;

    pub struct Input<'a> {
        input: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Input<'a> {
        fn new(source: &str) -> Input {
            Input {
                input: source.split_whitespace(),
            }
        }

        // pub fn tok(&mut self) -> &str {
        //     self.input.next().expect("getting token")
        // }
        // pub fn num(&mut self) -> isize {
        //     self.tok().parse().expect("parsing number")
        // }

        pub fn iter_num(self) -> impl Iterator<Item = isize> + 'a {
            self.input.map(|t| t.parse().expect("parsing number"))
        }
    }

    pub fn assert_level<F>(fun: F, input: &str, expected: &str)
    where
        F: Fn(Input) -> String,
    {
        let actual = fun(Input::new(input));
        assert_eq!(actual.trim(), expected.trim())
    }

    pub fn process_level<F>(fun: F, level: usize, part: usize)
    where
        F: Fn(Input) -> String,
    {
        let input =
            fs::read_to_string(format!("./input/level{}.txt", level)).expect("reading input");
        let output = fun(Input::new(&input));
        println!("level {}, part {}: {}", level, part, output);
    }
}

use boilerplate::{assert_level, process_level};

/// # Level1
mod level1 {
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

    pub fn part1(input: Input) -> String {
        input
            .iter_num()
            .map(fuel_for_mass)
            .sum::<isize>()
            .to_string()
    }

    pub fn part2(input: Input) -> String {
        input
            .iter_num()
            .map(fuel_for_mass_recursive)
            .sum::<isize>()
            .to_string()
    }
}

fn main() {
    assert_level(
        level1::part1,
        "12 14 1969 100756",
        &(2 + 2 + 654 + 33583).to_string(),
    );
    process_level(level1::part1, 1, 1);

    assert_level(
        level1::part2,
        "14 1969 100756",
        &(2 + 966 + 50346).to_string(),
    );
    process_level(level1::part2, 1, 2);
}
