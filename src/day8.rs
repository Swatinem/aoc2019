#[test]
fn test_day8() {}

struct Dimensions {
    width: usize,
    height: usize,
}

struct Image {
    // dimensions: Dimensions,
    layers: Vec<Vec<isize>>,
}

fn read_image(input: &str, dimensions: Dimensions) -> Image {
    let mut layers = vec![];

    let mut current_layer = vec![];

    for c in input.chars() {
        current_layer.push(c.to_digit(10).unwrap() as isize);
        if current_layer.len() == dimensions.width * dimensions.height {
            layers.push(current_layer);
            current_layer = vec![];
        }
    }

    Image {
        // dimensions,
        layers,
    }
}

fn composite(front_layer: &mut [isize], back_layer: &[isize]) {
    for (f, b) in front_layer.iter_mut().zip(back_layer.iter()) {
        *f = match (*f, *b) {
            (2, b) => b,
            (f, _) => f,
        };
    }
}

pub fn part1(input: &str) -> String {
    let mut image = read_image(
        input,
        Dimensions {
            width: 25,
            height: 6,
        },
    );

    // sort layers by number of zero digits
    image
        .layers
        .sort_by_cached_key(|l| l.iter().filter(|n| **n == 0).count());
    let layer = &image.layers[0];

    // the number of 1 digits multiplied by the number of 2 digits
    (layer.iter().filter(|n| **n == 1).count() * layer.iter().filter(|n| **n == 2).count())
        .to_string()
}

pub fn part2(input: &str) -> String {
    let image = read_image(
        input,
        Dimensions {
            width: 25,
            height: 6,
        },
    );

    let mut decoded = vec![2; 25 * 6];
    for layer in image.layers {
        composite(&mut decoded, &layer);
    }

    let mut path = std::path::PathBuf::new();
    path.push("output");
    path.push("level8.pbm");
    let mut pbm = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("opening pbm");

    use std::io::Write;
    writeln!(pbm, "P1").unwrap();
    writeln!(pbm, "25 6").unwrap();
    for n in decoded {
        write!(pbm, "{}", n).unwrap();
    }

    format!("image written to {:?}", path)
}
