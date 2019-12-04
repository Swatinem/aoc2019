mod day1;
mod day2;
mod day3;
mod day4;

pub fn process_level<F>(fun: F, level: usize, part: usize)
where
    F: Fn(&str) -> String,
{
    let input =
        std::fs::read_to_string(format!("./input/level{}.txt", level)).expect("reading input");
    let output = fun(input.trim());
    println!("level {}, part {}: {}", level, part, output);
}

fn main() {
    process_level(day1::part1, 1, 1);
    process_level(day1::part2, 1, 2);

    process_level(day2::part1, 2, 1);
    process_level(day2::part2, 2, 2);

    process_level(day3::part1, 3, 1);
    process_level(day3::part2, 3, 2);

    process_level(day4::part1, 4, 1);
    process_level(day4::part2, 4, 2);
}
