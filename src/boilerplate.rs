//! # Some Boilerplate
//! * An input tokenizer
//! * Directly testing a level on some fixtures
//! * Processing a level from an external input file
use std::fs;

pub struct Input<'a> {
    input: std::str::SplitWhitespace<'a>,
}

impl<'a> Input<'a> {
    pub fn new(source: &str) -> Input {
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
    F: Fn(&str) -> String,
{
    let actual = fun(input);
    assert_eq!(actual.trim(), expected.trim())
}

pub fn process_level<F>(fun: F, level: usize, part: usize)
where
    F: Fn(&str) -> String,
{
    let input = fs::read_to_string(format!("./input/level{}.txt", level)).expect("reading input");
    let output = fun(&input);
    println!("level {}, part {}: {}", level, part, output);
}
