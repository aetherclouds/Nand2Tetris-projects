mod parser;
mod transpile;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path}, collections::HashMap, str::FromStr,
    env::args
};

use crate::transpile::Transpiler;
use crate::parser::Parser;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(&filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    
    ///// PARSE INTRO ARRAY OF STRINGS
    let cmd_args = args().collect::<Vec<String>>();
    // 1st arg is always cwd, so we get the 2nd
    let path_arg = Path::new(cmd_args.get(1).expect("no runtime argument was provided!"));
    let filename = path_arg.file_stem().unwrap().to_str().unwrap();
    
    let instructions = lines_from_file(&path_arg);
    let mut parser = Parser::default();
    let commands = parser.parse_str_vec_to_cmd(&mut instructions.iter().map(|x|x.as_str()));
    print!("{:#?}", commands);

    let mut transpiler = Transpiler::default();
    transpiler.transp_cmd_vec_to_str(&commands);
}