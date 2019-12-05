enum Parameter {
    Position(usize),
    Immediate(isize),
}

impl Parameter {
    fn read(&self, memory: &[isize]) -> isize {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(addr) => memory[*addr],
        }
    }
    fn write(&self, memory: &mut [isize], value: isize) {
        match self {
            Parameter::Position(addr) => memory[*addr] = value,
            _ => panic!("writing to an Immediate parameter"),
        }
    }
}

enum ArithmeticOp {
    Add,
    Multiply,
}

enum JumpOp {
    IfNotZero,
    IfZero,
}

enum RelationOp {
    LessThan,
    Equal,
}

enum Instruction {
    Arithmetic(ArithmeticOp, Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    Jump(JumpOp, Parameter, Parameter),
    Relation(RelationOp, Parameter, Parameter, Parameter),
    Halt,
}

enum InstructionResult {
    Normal(usize),
    Jump(usize),
    Halt,
}

impl Instruction {
    fn decode(memory: &[isize], offset: usize) -> Instruction {
        let code = memory[offset] % 100;
        let param_modes = memory[offset] / 100;

        let decode_param = |n: u32| {
            let value = memory[offset + n as usize];
            let mode = param_modes / (10isize.pow(n - 1)) % 10;
            if mode == 1 {
                Parameter::Immediate(value)
            } else {
                Parameter::Position(value as usize)
            }
        };

        match code {
            1 | 2 => {
                let op = if code == 1 {
                    ArithmeticOp::Add
                } else {
                    ArithmeticOp::Multiply
                };
                Instruction::Arithmetic(op, decode_param(1), decode_param(2), decode_param(3))
            }
            3 => Instruction::Input(decode_param(1)),
            4 => Instruction::Output(decode_param(1)),
            5 | 6 => {
                let op = if code == 5 {
                    JumpOp::IfNotZero
                } else {
                    JumpOp::IfZero
                };
                Instruction::Jump(op, decode_param(1), decode_param(2))
            }
            7 | 8 => {
                let op = if code == 7 {
                    RelationOp::LessThan
                } else {
                    RelationOp::Equal
                };
                Instruction::Relation(op, decode_param(1), decode_param(2), decode_param(3))
            }
            99 => Instruction::Halt,
            _ => panic!("invalid opcode"),
        }
    }

    fn execute(&self, computer: &mut Computer) -> InstructionResult {
        match self {
            Instruction::Halt => InstructionResult::Halt,
            Instruction::Input(p) => {
                p.write(&mut computer.memory, computer.input);
                InstructionResult::Normal(2)
            }
            Instruction::Output(p) => {
                computer.output = p.read(&computer.memory);
                InstructionResult::Normal(2)
            }
            Instruction::Arithmetic(op, lhs, rhs, dst) => {
                let lhs = lhs.read(&computer.memory);
                let rhs = rhs.read(&computer.memory);
                let result = match op {
                    ArithmeticOp::Add => lhs + rhs,
                    ArithmeticOp::Multiply => lhs * rhs,
                };
                dst.write(&mut computer.memory, result);
                InstructionResult::Normal(4)
            }
            Instruction::Relation(op, lhs, rhs, dst) => {
                let lhs = lhs.read(&computer.memory);
                let rhs = rhs.read(&computer.memory);
                let result = match op {
                    RelationOp::Equal if lhs == rhs => 1,
                    RelationOp::LessThan if lhs < rhs => 1,
                    _ => 0,
                };
                dst.write(&mut computer.memory, result);
                InstructionResult::Normal(4)
            }
            Instruction::Jump(op, value, jump) => {
                let value = value.read(&computer.memory);
                let jump = jump.read(&computer.memory) as usize;
                match op {
                    JumpOp::IfNotZero if value != 0 => InstructionResult::Jump(jump),
                    JumpOp::IfZero if value == 0 => InstructionResult::Jump(jump),
                    _ => InstructionResult::Normal(3),
                }
            }
        }
    }
}

struct Computer {
    memory: Vec<isize>,
    input: isize,
    output: isize,
}

impl Computer {
    fn new(memory: &str) -> Computer {
        let memory = memory
            .split(',')
            .map(|n| n.parse::<isize>().expect("parsing number"))
            .collect();

        Computer {
            memory,
            input: 0,
            output: 0,
        }
    }

    fn new_with_input(memory: &str, input: isize) -> Computer {
        let mut c = Self::new(memory);
        c.input = input;
        c
    }

    fn run(&mut self) -> isize {
        let mut pc = 0;
        loop {
            let inst = Instruction::decode(&self.memory, pc);
            let result = inst.execute(self);
            match result {
                InstructionResult::Halt => return self.output,
                InstructionResult::Normal(width) => pc += width,
                InstructionResult::Jump(to) => pc = to,
            }
        }
    }
}

#[test]
fn test_day5() {
    fn run_test(mem: &str, input: isize) -> isize {
        let mut c = Computer::new_with_input(mem, input);
        c.run()
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
    let result = c.run();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 5);
    let result = c.run();
    result.to_string()
}
