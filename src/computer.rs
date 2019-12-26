use std::collections::VecDeque;

enum Parameter {
    Position(usize),
    Immediate(isize),
    Relative(isize),
}

impl Parameter {
    fn read(&self, computer: &Computer) -> isize {
        let addr = match self {
            Parameter::Immediate(value) => return *value,
            Parameter::Position(addr) => *addr,
            Parameter::Relative(offset) => (computer.relative_base + *offset) as usize,
        };
        return computer.memory.get(addr).map(|x| *x).unwrap_or(0);
    }
    fn write(&self, computer: &mut Computer, value: isize) {
        let addr = match self {
            Parameter::Position(addr) => *addr,
            Parameter::Relative(offset) => (computer.relative_base + *offset) as usize,
            _ => panic!("writing to an Immediate parameter"),
        };
        computer
            .memory
            .resize(computer.memory.len().max(addr + 1), 0);
        computer.memory[addr] = value;
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
    AdjustBase(Parameter),
    Halt,
}

enum InstructionResult {
    Waiting,
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
            match mode {
                0 => Parameter::Position(value as usize),
                1 => Parameter::Immediate(value),
                2 => Parameter::Relative(value as isize),
                _ => panic!("unknown parameter mode"),
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
            9 => Instruction::AdjustBase(decode_param(1)),
            99 => Instruction::Halt,
            _ => panic!("invalid opcode"),
        }
    }

    fn execute(&self, computer: &mut Computer) -> InstructionResult {
        match self {
            Instruction::Halt => InstructionResult::Halt,
            Instruction::Input(p) => match computer.input.pop_front() {
                Some(value) => {
                    p.write(computer, value);
                    InstructionResult::Normal(2)
                }
                None => InstructionResult::Waiting,
            },
            Instruction::Output(p) => {
                computer.output.push_back(p.read(computer));
                InstructionResult::Normal(2)
            }
            Instruction::Arithmetic(op, lhs, rhs, dst) => {
                let lhs = lhs.read(computer);
                let rhs = rhs.read(computer);
                let result = match op {
                    ArithmeticOp::Add => lhs + rhs,
                    ArithmeticOp::Multiply => lhs * rhs,
                };
                dst.write(computer, result);
                InstructionResult::Normal(4)
            }
            Instruction::Relation(op, lhs, rhs, dst) => {
                let lhs = lhs.read(computer);
                let rhs = rhs.read(computer);
                let result = match op {
                    RelationOp::Equal if lhs == rhs => 1,
                    RelationOp::LessThan if lhs < rhs => 1,
                    _ => 0,
                };
                dst.write(computer, result);
                InstructionResult::Normal(4)
            }
            Instruction::Jump(op, value, jump) => {
                let value = value.read(computer);
                let jump = jump.read(computer) as usize;
                match op {
                    JumpOp::IfNotZero if value != 0 => InstructionResult::Jump(jump),
                    JumpOp::IfZero if value == 0 => InstructionResult::Jump(jump),
                    _ => InstructionResult::Normal(3),
                }
            }
            Instruction::AdjustBase(value) => {
                computer.relative_base += value.read(computer);
                InstructionResult::Normal(2)
            }
        }
    }
}

pub enum RunResult {
    Completed(isize),
    Waiting,
}

#[derive(Clone)]
pub struct Computer {
    memory: Vec<isize>,
    pc: usize,
    relative_base: isize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}

impl Computer {
    pub fn new(memory: &str) -> Computer {
        let memory = memory
            .split(',')
            .map(|n| n.parse::<isize>().expect("parsing number"))
            .collect();

        Computer {
            memory,
            pc: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn new_with_input(memory: &str, input: isize) -> Computer {
        let mut c = Self::new(memory);
        c.input.push_back(input);
        c
    }

    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    pub fn pop_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    #[cfg(test)]
    pub fn output(&self) -> &VecDeque<isize> {
        &self.output
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            let inst = Instruction::decode(&self.memory, self.pc);
            let result = inst.execute(self);
            match result {
                InstructionResult::Waiting => return RunResult::Waiting,
                InstructionResult::Halt => {
                    let most_recent_output = *self.output.back().expect("halting without output");

                    return RunResult::Completed(most_recent_output);
                }
                InstructionResult::Normal(width) => self.pc += width,
                InstructionResult::Jump(to) => self.pc = to,
            }
        }
    }

    pub fn run_to_halt(&mut self) -> isize {
        match self.run() {
            RunResult::Completed(value) => value,
            RunResult::Waiting => panic!("expected computer to complete"),
        }
    }
}
