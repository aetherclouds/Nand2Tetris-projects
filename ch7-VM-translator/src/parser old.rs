use std::{path::PrefixComponent, os::windows::process};


enum Command {
    StackOperation {operationType: OperationType, segment: MemorySegment, index: i16},
    ArithmeticOperation {operationType: OperationType},
}

// this became irrelevant since every operationType is linked to one CommandType, whereupon it can be inferred
// enum CommandType {
//     StackOperation,
//     ArithmeticOperation
// }

enum MemorySegment {
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

enum OperationType {
    // stack
    Pop,
    Push,
    // arithmetic
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

// // this was my guiding light: https://towardsdatascience.com/write-better-match-statements-in-rust-7458402afacd
// fn process_cmd_but_old(operation_type: OperationType, rest_of_iter: Iterator<&str>) -> Option<Command> {
//     Some(match operation_type {
//         // stack
//         stackOperationType @ OperationType::Pop | OperationType::Push => process_stack_cmd(stackOperationType, rest_of_iter),
//         OperationType::Pop => process_stack_cmd(OperationType::Push, rest_of_iter),
//         // arithmetic
//         _ => {Command::ArithmeticOperation { operationType: _ }},
//     })

// }

fn process_cmd(operation_type: OperationType, rest_of_iter: impl Iterator<Item = &str>) -> Option<Command> {
    match match_operation_type(rest_of_iter.next().unwrap()) {
    }
    Some(match operation_type {
        // stack
        stackOperationType @ OperationType::Pop | OperationType::Push => process_stack_cmd(stackOperationType, rest_of_iter),
        // arithmetic
        _ => panic!(),
    })

}

fn process_stack_cmd(operation_type: OperationType, rest_of_iter: impl Iterator<Item = &str>) {
    let segment = rest_of_iter.next().expect("Segment parameter missing!");
    let idx: i16 = rest_of_iter.next().expect("Index parameter missing!").parse();
    match operation_type {
        OperationType::Push => Command::StackOperation { operationType: OperationType::Push, segment: match_segment(segment), index: idx },
        OperationType::Pop => Command::StackOperation { operationType: OperationType::Pop, segment: match_segment(segment), index: idx },
        @
        _ => panic!("you must've done something really dumb there")
    }
}

fn match_operation_type(string: &str) -> OperationType {
    match string {
        "push" => OperationType::Push,
        "pop" => OperationType::Pop,
        "add" => OperationType::ADD,
        "subtract" => OperationType::SUB,
        "negate" => OperationType::NEG,
        "equal" => OperationType::EQ,
        "gt" => OperationType::GT,
        "lt" => OperationType::LT,
        "and" => OperationType::AND,
        "or" => OperationType::OR,
        "not" => OperationType::NOT,
        _ => panic!("invalid operation type!")
    }
}

fn match_segment(string: &str) -> MemorySegment {
    match string {
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
        _ => panic!("Invalid segment!")
    }
}

pub fn process_string_to_cmd(command: &String) -> Option<Command> {

    let words: Vec<&str> = command.split(" ").collect();
    let mut words_iter = words.iter().map(|x| *x);
    let cmd = process_cmd(
        match_operation_type(words_iter.next().unwrap()), 
        words_iter
    );
    return cmd
}

pub fn process_string_arr_to_cmd(command_arr: impl Iterator<Item = &str>)