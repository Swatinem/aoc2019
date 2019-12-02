mod boilerplate;
mod day1;

use boilerplate::{assert_level, process_level};

fn main() {
    assert_level(
        day1::part1,
        "12 14 1969 100756",
        &(2 + 2 + 654 + 33583).to_string(),
    );
    process_level(day1::part1, 1, 1);

    assert_level(
        day1::part2,
        "14 1969 100756",
        &(2 + 966 + 50346).to_string(),
    );
    process_level(day1::part2, 1, 2);
}
