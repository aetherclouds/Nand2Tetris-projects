use std::{fmt::format, vec};
use crate::parser::{self, MemorySegment};

// const LCL_ADDR: str = "LCL";

// the 2 are inversed because the stack starts on the top of the RAM and goes down
const incr_sp_str: &str = "M=M-1\n";
const decr_sp_str: &str = "M=M+1\n";

pub struct Transpiler {
    // stackBase: u16,
}

fn go_to_segment(mem_segment: parser::MemorySegment, index: u16) -> String {
    match_segment(&mem_segment).map_or_else(
        || format!("@{index}"), 
        |segment| format!("@{segment}\nA=A+{index}")
)
}

pub fn match_segment(mem_segment: &parser::MemorySegment) -> Option<&'static str> {
    match mem_segment {
        parser::MemorySegment::SP =>    Some("SP"),
        parser::MemorySegment::LCL =>   Some("LCL"),
        parser::MemorySegment::ARG =>   Some("ARG"),
        parser::MemorySegment::THIS =>  Some("THIS"),
        parser::MemorySegment::THAT =>  Some("THAT"),
        parser::MemorySegment::TEMP =>  Some("TEMP"),
        parser::MemorySegment::R13 =>   Some("R13"),
        parser::MemorySegment::R14 =>   Some("R14"),
        parser::MemorySegment::R15 =>   Some("R15"),
        parser::MemorySegment::CONST => None,
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
        }
    }
}

impl Transpiler {
    fn init(&self) {
        self.write_line("// TRANSPILED FILE");
        // self.write_line("// INITIALIZING\n@SP\nM=")
        // TODO: write stack pointers
    }

    fn exit(&self) {
        self.write_line("// EXIT\n@EXIT_LOOP\n0;JMP\n(EXIT_LOOP)\n@EXIT_LOOP\n0;JMP")
    }

    fn write_line(&self, line: &str) { // this method can be an abstraction -- for a start we write to the console
        println!("{}", line)
    }

    fn write_cmd(&self, cmd: &parser::Command) {
        self.write_line(format!("// {}", &cmd.cmd_as_str).as_str());
        self.write_line(self.transp_cmd_to_str(&cmd.cmd_as_obj).as_str());
    }

    fn transp_cmd_to_str(&self, cmd: &parser::CommandAsObject) -> String {
        
        let mut out_string = String::new();
        match cmd {
            parser::CommandAsObject::StackCommand { operationType, memorySegment, index } => {
                // -- for POP:
                // @SP
                // - decrement stack pointer
                // M=M-1    
                // A=M+1 - move to addr referenced by sp (we have to compensate for the decrement)
                // D=M - get data from memory
                // - now we gotta do the following: @{SEGMENT+INDEX}
                // @{SEGMENT}
                // A=A+{INDEX}
                // M=D

                // -- for PUSH:
                // - get data from SEGMENT+INDEX 
                // @{SEGMENT}
                // A=A+{INDEX}
                // D=M
                // - ..then go to top of the stack
                // @SP
                // - increment stack pointer
                // M=M+1
                // A=M - move to addr referenced by sp
                // M=D - then finally "push" data to stack

                
                let index_as_str = index.to_string();
                out_string.push_str(match operationType {
                    parser::StackOperationType::Pop => {
                        match_segment(memorySegment).map_or_else(
                            ||panic!("DON'T USE `CONST` ON A `POP` COMMAND WTF"), 
                            |mem_segment|
                            format!("@SP\nM=M-1\nA=M+1\nD=M\n@{mem_segment}\nA=A+{index}\nM=D")
                        )
                    },
                    parser::StackOperationType::Push => {
                        match_segment(memorySegment).map_or_else(
                            || // if memorySegment == CONST
                            format!("@{index}\nD=A\n@SP\nM=M+1\nA=M\nM=D"), 
                            |mem_segment|
                            format!("@{mem_segment}\nA=A+{index}\nD=M\n@SP\nM=M+1\nA=M\nM=D")
                        )
                    },
                }.as_str());
            },
            parser::CommandAsObject::ArithmeticCommand { operationType } => {
                out_string.push_str("@SP\nM=M-1\nA=M+1\nD=M\nA=A-1\n"); 
                // 1st operand: M (@SP)
                // 2nd operand: D (@SP+1)
                // output -> M
                out_string.push_str(
            match operationType {
                        parser::ArithmeticOperationType::ADD =>  "M=M+D",
                        parser::ArithmeticOperationType::SUB =>  "M=M-D",
                        parser::ArithmeticOperationType::NEG =>  "M=0-M",
                        parser::ArithmeticOperationType::EQ =>   "",
                        parser::ArithmeticOperationType::GT =>   "",
                        parser::ArithmeticOperationType::LT =>   "",
                        parser::ArithmeticOperationType::AND =>  "",
                        parser::ArithmeticOperationType::OR =>   "",
                        parser::ArithmeticOperationType::NOT =>  "",

                    }
                )
            },
        }
        out_string
    }

    pub fn transp_cmd_vec_to_str(&self, cmd_vec: &Vec<parser::Command>) { // could maybe return file
        self.init();
        for cmd in cmd_vec {
            self.write_cmd(cmd);
        }
        self.exit();

    }
}