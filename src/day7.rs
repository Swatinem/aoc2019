use crate::computer::{Computer, RunResult};

#[test]
fn test_day7() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        "43210"
    );
    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        "54321"
    );
    assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), "65210");

    assert_eq!(
        part2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        "139629729"
    );
    assert_eq!(
        part2(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        ),
        "18216"
    );
}

// basically copy-pasta of https://en.wikipedia.org/wiki/Heap%27s_algorithm#Details_of_the_algorithm
fn permute<O>(mut output: O, mut a: Vec<isize>)
where
    O: FnMut(Vec<isize>) -> (),
{
    let n = a.len();
    //c is an encoding of the stack state. c[k] encodes the for-loop counter for when generate(k+1, A) is called
    let mut c = vec![0; n];

    output(a.clone());

    //i acts similarly to the stack pointer
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if (i % 2) == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            output(a.clone());
            //Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            //Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 0;
        } else {
            //Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1;
        }
    }
}

fn generate_permutations(input: Vec<isize>) -> Vec<Vec<isize>> {
    let mut permutations = vec![];
    permute(&mut |o| permutations.push(o), input);
    permutations
}

pub fn part1(input: &str) -> String {
    // create the computers
    let amplifiers = vec![Computer::new(input); 5];
    let mut max_output = 0;

    for permutation in generate_permutations((0..5).collect()) {
        // make sure to use fresh state
        let mut amplifiers = amplifiers.clone();
        let mut permutation = permutation;
        // provide the sequence number as first input
        for amplifier in &mut amplifiers {
            amplifier.push_input(permutation.pop().unwrap());
        }

        // then run all the stuff, back to front
        let mut current_output = 0;
        while !amplifiers.is_empty() {
            let mut amplifier = amplifiers.pop().unwrap();
            amplifier.push_input(current_output);
            current_output = match amplifier.run() {
                RunResult::Completed(value) => value,
                RunResult::Waiting => panic!("expected computer to complete"),
            };
        }

        max_output = max_output.max(current_output);
    }

    max_output.to_string()
}

pub fn part2(input: &str) -> String {
    // create the computers
    let amplifiers = vec![Computer::new(input); 5];
    let mut max_output = 0;

    for permutation in generate_permutations((5..10).collect()) {
        // make sure to use fresh state
        let mut amplifiers = amplifiers.clone();
        let mut permutation = permutation;
        // provide the sequence number as first input
        for amplifier in &mut amplifiers {
            amplifier.push_input(permutation.pop().unwrap());
        }

        // then run all the stuff, back to front
        let mut current_output = 0;
        let mut current_amplifier = 0;
        loop {
            let amplifier = &mut amplifiers[current_amplifier];
            amplifier.push_input(current_output);
            current_amplifier += 1;
            current_output = match amplifier.run() {
                RunResult::Completed(value) => {
                    // if *all* the amplifiers are complete, stop the feedback loop
                    if current_amplifier == amplifiers.len() {
                        current_output = value;
                        break;
                    }
                    value
                }
                RunResult::Waiting => amplifier.get_output().expect("expected outout"),
            };
            // start from the front again
            current_amplifier = current_amplifier % amplifiers.len();
        }

        max_output = max_output.max(current_output);
    }

    max_output.to_string()
}
