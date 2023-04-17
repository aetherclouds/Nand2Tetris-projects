#[derive(Debug)]
pub enum Command {
    StackCommand {operationType: StackOperationType, memorySegment: MemorySegment, index: i16},
    ArithmeticCommand {operationType: ArithmeticOperationType},
}

// I thought this would become irrelevant since every operationType is linked to one CommandType, whereupon it can be inferred
#[derive(Debug)]
pub enum MemorySegment {
    SP,
    LCL,
    ARG,
    THIS,
    THAT,
    TEMP,
    R13,
    R14,
    R15,
    // virtual segment
    CONST,
}

#[derive(Debug)]
pub enum OperationType {
    StackOperationType(StackOperationType),
    ArithmeticOperationType(ArithmeticOperationType),
}

#[derive(Debug)]
pub enum StackOperationType {
    Pop,
    Push,
}

#[derive(Debug)]
pub enum ArithmeticOperationType {
    ADD,
    SUB,
    NEG,

    EQ,
    GT,
    LT,

    AND,
    OR,
    NOT,
}

pub struct Parser {
    current_line: u32
}


impl Default for Parser {
    fn default() -> Self {
        Self {
            current_line: 0,
        }
    }
}

// btw, Self is the "type" of self
// self is the actual self variable
// you can call methods like `Self::method(self, args)`
// or `self.method(args)` where the `self` parameter is inferred
impl Parser {

    // fn match_operation_type(&self, string: &str) -> OperationType {
    //     match string {
    //         "push" => OperationType::Push,
    //         "pop" => OperationType::Pop,
    //         "add" => OperationType::ADD,
    //         "subtract" => OperationType::SUB,
    //         "negate" => OperationType::NEG,
    //         "equal" => OperationType::EQ,
    //         "gt" => OperationType::GT,
    //         "lt" => OperationType::LT,
    //         "and" => OperationType::AND,
    //         "or" => OperationType::OR,
    //         "not" => OperationType::NOT,
    //         _ => self.parser_panic("invalid operation type!")
    //     }
    // }

    fn match_segment(&self, word: &str) -> MemorySegment {
        match word {
            "pointer" => MemorySegment::SP,
            "local" => MemorySegment::LCL,
            "argument" => MemorySegment::ARG,
            "this" => MemorySegment::THIS,
            "that" => MemorySegment::THAT,
            "temp" => MemorySegment::TEMP,
            // "r13" => MemorySegment::R13,
            // "r14" => MemorySegment::R14,
            // "r15" => MemorySegment::R15,
            "constant" => MemorySegment::CONST,
            _ => self.parser_panic(format!("Invalid segment `{}`", word).as_str())
        }
    }
    
    fn parse_operation_type(&self, word: &str) -> OperationType {
        match word {
            "push" =>       OperationType::StackOperationType(StackOperationType::Push),
            "pop" =>        OperationType::StackOperationType(StackOperationType::Pop),
            "add" =>        OperationType::ArithmeticOperationType(ArithmeticOperationType::ADD),
            "sub" =>        OperationType::ArithmeticOperationType(ArithmeticOperationType::SUB),
            "negate" =>     OperationType::ArithmeticOperationType(ArithmeticOperationType::NEG),
            "equal" =>      OperationType::ArithmeticOperationType(ArithmeticOperationType::EQ),
            "gt" =>         OperationType::ArithmeticOperationType(ArithmeticOperationType::GT),
            "lt" =>         OperationType::ArithmeticOperationType(ArithmeticOperationType::LT),
            "and" =>        OperationType::ArithmeticOperationType(ArithmeticOperationType::AND),
            "or" =>         OperationType::ArithmeticOperationType(ArithmeticOperationType::OR),
            "not" =>        OperationType::ArithmeticOperationType(ArithmeticOperationType::NOT),
            _ => self.parser_panic(format!("Invalid operation `{}`", word).as_str())
        }
    }
    
    fn make_cmd<'a>(&self, operation_type: OperationType, rest_of_iter: &mut impl Iterator<Item = &'a str>) -> Command {
        match operation_type {
            OperationType::StackOperationType(op_type) => {
                Command::StackCommand { 
                    operationType: op_type, 
                    memorySegment: self.match_segment(rest_of_iter.next().unwrap()), 
                    index: (rest_of_iter.next().unwrap().parse().unwrap()) 
                }
            },
            OperationType::ArithmeticOperationType(op_type) => Command::ArithmeticCommand { operationType: op_type },
        }
    }
    
    pub fn parse_str_to_cmd(&mut self, string: &str) -> Option<Command> {
        self.current_line += 1;
        if string.chars().count() > 0 && !string.starts_with("//") {
            let words: Vec<&str> = string.split(" ").collect();
            let mut words_iter = words.iter().map(|x| *x);
            let operation_type = self.parse_operation_type(words_iter.next().unwrap());
            Some(self.make_cmd(
                operation_type,
                words_iter.by_ref()
            ))
        } else {None}
    }

    pub fn parse_str_vec_to_cmd<'a>(&mut self, string_list: &mut impl Iterator<Item = &'a str>) -> Vec<Command> {
        let mut cmds: Vec<Command> = vec![];   
        for line in string_list {
            let cmd_option = self.parse_str_to_cmd(line);
            match cmd_option {
                Some(cmd) => {
                    cmds.push(cmd);
                },
                None => {continue;},
            }
        }
        cmds 
    }

    fn parser_panic(&self, message: &str) -> ! {
        panic!("line {}: {message}", self.current_line.to_string());
    }
    
}
