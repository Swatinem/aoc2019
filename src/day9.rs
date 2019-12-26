use crate::computer::Computer;

#[test]
fn test_day9() {
    fn run_test(mem: &str) -> Vec<isize> {
        let mut c = Computer::new(mem);
        c.run_to_halt();
        c.output().iter().cloned().collect()
    }

    assert_eq!(
        run_test("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
    assert_eq!(
        run_test("1102,34915192,34915192,7,4,7,99,0")[0]
            .to_string()
            .len(),
        16
    );
    assert_eq!(run_test("104,1125899906842624,99"), vec![1125899906842624]);
}

pub fn part1(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 1);
    let result = c.run_to_halt();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 2);
    let result = c.run_to_halt();
    result.to_string()
}
