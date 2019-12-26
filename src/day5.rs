use crate::computer::Computer;

#[test]
fn test_day5() {
    fn run_test(mem: &str, input: isize) -> isize {
        let mut c = Computer::new_with_input(mem, input);
        c.run_to_halt()
    }

    // equal to 8
    assert_eq!(run_test("3,9,8,9,10,9,4,9,99,-1,8", 7), 0);
    assert_eq!(run_test("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
    assert_eq!(run_test("3,9,8,9,10,9,4,9,99,-1,8", 9), 0);

    // less than 8
    assert_eq!(run_test("3,9,7,9,10,9,4,9,99,-1,8", 7), 1);
    assert_eq!(run_test("3,9,7,9,10,9,4,9,99,-1,8", 8), 0);
    assert_eq!(run_test("3,9,7,9,10,9,4,9,99,-1,8", 9), 0);

    // equal to 8
    assert_eq!(run_test("3,3,1108,-1,8,3,4,3,99", 7), 0);
    assert_eq!(run_test("3,3,1108,-1,8,3,4,3,99", 8), 1);
    assert_eq!(run_test("3,3,1108,-1,8,3,4,3,99", 9), 0);

    // less than 8
    assert_eq!(run_test("3,3,1107,-1,8,3,4,3,99", 7), 1);
    assert_eq!(run_test("3,3,1107,-1,8,3,4,3,99", 8), 0);
    assert_eq!(run_test("3,3,1107,-1,8,3,4,3,99", 9), 0);

    // zero / nonzero
    assert_eq!(run_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), 0);
    assert_eq!(run_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 100), 1);
    assert_eq!(run_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), 0);
    assert_eq!(run_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 100), 1);

    // more complex one…
    let complex = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(run_test(complex, 7), 999);
    assert_eq!(run_test(complex, 8), 1000);
    assert_eq!(run_test(complex, 9), 1001);
}

pub fn part1(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 1);
    let result = c.run_to_halt();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 5);
    let result = c.run_to_halt();
    result.to_string()
}
