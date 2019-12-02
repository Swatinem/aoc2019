enum OpCode {
    // Add = 1,
// Multiply = 2,
// End = 99,
}

impl OpCode {
    fn execute_at(memory: &mut [usize], pc: usize) -> usize {
        // TODO: maybe convert this to a real enum?
        let opcode = memory[pc];
        // meh… can I simplify this somehow?
        match opcode {
            1 => {
                if let [1, lhs, rhs, ret] = memory[pc..pc + 4] {
                    memory[ret] = memory[lhs] + memory[rhs];
                }
                4
            }
            2 => {
                if let [2, lhs, rhs, ret] = memory[pc..pc + 4] {
                    memory[ret] = memory[lhs] * memory[rhs];
                }
                4
            }
            _ => 0,
        }
    }
}

fn execute_program(memory: &mut [usize]) -> &[usize] {
    let mut pc = 0;
    loop {
        let offset = OpCode::execute_at(memory, pc);
        if offset == 0 {
            return memory;
        }
        pc += offset;
    }
}

#[test]
fn test_part1() {
    let mut mem = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(
        execute_program(&mut mem),
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );

    let mut mem = vec![1, 0, 0, 0, 99];
    assert_eq!(execute_program(&mut mem), &[2, 0, 0, 0, 99]);

    let mut mem = vec![2, 3, 0, 3, 99];
    assert_eq!(execute_program(&mut mem), &[2, 3, 0, 6, 99]);

    let mut mem = vec![2, 4, 4, 5, 99, 0];
    assert_eq!(execute_program(&mut mem), &[2, 4, 4, 5, 99, 9801]);

    let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    assert_eq!(execute_program(&mut mem), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

fn read_mem(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.parse::<usize>().expect("parsing number"))
        .collect()
}

pub fn part1(input: &str) -> String {
    let mut mem = read_mem(input);
    // before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    mem[1] = 12;
    mem[2] = 2;
    execute_program(&mut mem);
    // What value is left at position 0 after the program halts?
    mem[0].to_string()
}
