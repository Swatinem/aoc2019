#[test]
fn test_day4() {
    assert_eq!(is_valid_password("111111"), true);
    assert_eq!(is_valid_password("223450"), false);
    assert_eq!(is_valid_password("123789"), false);

    assert_eq!(is_valid_password2("112233"), true);
    assert_eq!(is_valid_password2("123444"), false);
    assert_eq!(is_valid_password2("111122"), true);
}

fn is_valid_password(input: &str) -> bool {
    let mut has_double = false;
    let mut last_char = input.chars().next().expect("expected one char");

    for c in input.chars().skip(1) {
        if c == last_char {
            has_double = true;
        } else if c < last_char {
            return false;
        }
        last_char = c;
    }
    has_double
}

fn is_valid_password2(input: &str) -> bool {
    // char, count
    let mut rle = vec![(input.chars().next().expect("expected one char"), 1)];
    let mut current = rle.last_mut().unwrap();

    for c in input.chars().skip(1) {
        if c == current.0 {
            current.1 += 1;
        } else if c < current.0 {
            return false;
        } else {
            rle.push((c, 1));
            current = rle.last_mut().unwrap();
        }
    }

    rle.iter().any(|pair| pair.1 == 2)
}

pub fn part1(input: &str) -> String {
    let mut input = input
        .split('-')
        .map(|d| d.parse::<isize>().expect("parsing number"));
    let from = input.next().expect("expected input");
    let to = input.next().expect("expected input");

    let mut valid_passwords = 0;
    for pw in from..=to {
        if is_valid_password(&pw.to_string()) {
            valid_passwords += 1;
        }
    }
    valid_passwords.to_string()
}

pub fn part2(input: &str) -> String {
    let mut input = input
        .split('-')
        .map(|d| d.parse::<isize>().expect("parsing number"));
    let from = input.next().expect("expected input");
    let to = input.next().expect("expected input");

    let mut valid_passwords = 0;
    for pw in from..=to {
        if is_valid_password2(&pw.to_string()) {
            valid_passwords += 1;
        }
    }
    valid_passwords.to_string()
}
