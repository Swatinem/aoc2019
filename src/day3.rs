use std::collections::HashMap;

#[test]
fn test_day3() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), "6");
    assert_eq!(
        part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "159"
    );
    assert_eq!(
        part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "135"
    );

    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), "30");
    assert_eq!(
        part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "610"
    );
    assert_eq!(
        part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "410"
    );
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("expected direction"),
        }
    }
}

struct Command {
    direction: Direction,
    length: isize,
}

type Path = Vec<Command>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

fn parse_path(input: &str) -> Path {
    input
        .split(',')
        .map(|c| Command {
            direction: c.chars().nth(0).expect("expected char").into(),
            length: c[1..].parse().expect("parsing number"),
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let mut input = input.lines();

    let path1 = input.next().map(parse_path).expect("parsing path");
    let path2 = input.next().map(parse_path).expect("parsing path");

    // So essentially, we need a sparse, infinitely growable matrix, hm…
    // lets just use a hashmap, best we can do right now
    let mut field = HashMap::new();
    let origin = Point { x: 0, y: 0 };
    field.insert(origin.clone(), 0);

    let mut min_distance = isize::max_value();

    let mut trace_path = |path: Path, id: usize| {
        let mut point = origin.clone();
        for cmd in &path {
            let (step_x, step_y) = match cmd.direction {
                Direction::Up => (1, 0),
                Direction::Right => (0, 1),
                Direction::Down => (-1, 0),
                Direction::Left => (0, -1),
            };
            for _ in 0..cmd.length {
                point.x += step_x;
                point.y += step_y;
                // record this path on the field
                let entry = field.entry(point.clone()).or_insert(id);
                // check to see if there is a *different* path already, in
                // which case this is an intersection
                if *entry > 0 && *entry != id {
                    min_distance = isize::min(min_distance, point.x.abs() + point.y.abs());
                }
            }
        }
    };

    trace_path(path1, 1);
    trace_path(path2, 2);

    min_distance.to_string()
}

pub fn part2(input: &str) -> String {
    let mut input = input.lines();

    let path1 = input.next().map(parse_path).expect("parsing path");
    let path2 = input.next().map(parse_path).expect("parsing path");

    // So essentially, we need a sparse, infinitely growable matrix, hm…
    // lets just use a hashmap, best we can do right now
    let mut field = HashMap::new();
    let origin = Point { x: 0, y: 0 };
    field.insert(origin.clone(), (0, 0));

    let mut min_steps = isize::max_value();

    let mut trace_path = |path: Path, id: usize| {
        let mut point = origin.clone();
        let mut path_length = 0;
        for cmd in &path {
            let (step_x, step_y) = match cmd.direction {
                Direction::Up => (1, 0),
                Direction::Right => (0, 1),
                Direction::Down => (-1, 0),
                Direction::Left => (0, -1),
            };
            for _ in 0..cmd.length {
                path_length += 1;
                point.x += step_x;
                point.y += step_y;
                // record this path on the field
                let entry = field.entry(point.clone()).or_insert((id, path_length));
                // check to see if there is a *different* path already, in
                // which case this is an intersection
                if entry.0 > 0 && entry.0 != id {
                    min_steps = isize::min(min_steps, entry.1 + path_length);
                }
            }
        }
    };

    trace_path(path1, 1);
    trace_path(path2, 2);

    min_steps.to_string()
}
