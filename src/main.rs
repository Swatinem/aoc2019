mod day1;
mod day2;

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
}
