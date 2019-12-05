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

enum Instruction {
    Arithmetic(ArithmeticOp, Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
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
            99 => Instruction::Halt,
            _ => panic!("invalid opcode"),
        }
    }

    fn execute(&self, computer: &mut Computer) -> usize {
        match self {
            Instruction::Halt => 0,
            Instruction::Input(p) => {
                p.write(&mut computer.memory, computer.input);
                2
            }
            Instruction::Output(p) => {
                computer.output = p.read(&computer.memory);
                2
            }
            Instruction::Arithmetic(op, lhs, rhs, dst) => {
                let lhs = lhs.read(&computer.memory);
                let rhs = rhs.read(&computer.memory);
                let result = match op {
                    ArithmeticOp::Add => lhs + rhs,
                    ArithmeticOp::Multiply => lhs * rhs,
                };
                dst.write(&mut computer.memory, result);
                4
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
            let width = inst.execute(self);
            if width == 0 {
                return self.output;
            }
            pc += width;
        }
    }
}

#[test]
fn test_day5() {}

pub fn part1(input: &str) -> String {
    let mut c = Computer::new_with_input(input, 1);
    let result = c.run();
    result.to_string()
}

pub fn part2(_input: &str) -> String {
    "".into()
}
