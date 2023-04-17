use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path}, collections::HashMap, str::FromStr,
};
use std::env::args;


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(&filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug)]
struct SaveCompTo{
    a: bool, 
    d: bool, 
    m: bool,
}
impl Default for SaveCompTo {
    fn default() -> Self {
        Self { a: false, d: false, m: false }
    }
}
#[derive(Debug)]
enum JumpIf{
    Null,
    GT,
    EQ,
    GE,
    LT,
    NE,
    LE,
    JMP
}
// X here will correspond to A or M since we can choose between either
// this also means that operations involving A AND M are forbidden
#[derive(Debug)]
enum Comp{
    Zero,
    One,
    NegOne,
    D,
    X,
    NotD,
    NotX,
    NegD,
    NegX,
    DPlusOne,
    XPlusOne,
    DMinusOne,
    XMinusOne,
    SumDAndX,
    DMinusX,
    XMinusD,
    DAndX,
    DOrX,
}

#[derive(Debug)]
struct CInstr {
    line: u16,
    switch_a_for_m: bool,
    comp: Comp,
    save_comp_to: SaveCompTo,
    jump_if: JumpIf,
}

impl CInstr {

    fn instr_as_binary(&self) -> String {
        let mut instr = String::from_str("111").unwrap();
        instr.push(if self.switch_a_for_m {'1'} else {'0'});
        instr.push_str(match self.comp {
            Comp::Zero => "101010",
            Comp::One => "111111",
            Comp::NegOne => "111010",
            Comp::D => "001100",
            Comp::X => "110000",
            Comp::NotD => "001101",
            Comp::NotX => "110001",
            Comp::NegD => "001111",
            Comp::NegX => "110011",
            Comp::DPlusOne => "011111",
            Comp::XPlusOne => "110111",
            Comp::DMinusOne => "001110",
            Comp::XMinusOne => "110010",
            Comp::SumDAndX => "000010",
            Comp::DMinusX => "010011",
            Comp::XMinusD => "000111",
            Comp::DAndX => "000000",
            Comp::DOrX => "010101",
        });
        instr.push_str({
            let mut save_instr = String::new();
            save_instr.push(if self.save_comp_to.a{'1'} else {'0'});
            save_instr.push(if self.save_comp_to.d{'1'} else {'0'});
            save_instr.push(if self.save_comp_to.m{'1'} else {'0'});
            &save_instr.to_owned()
        });
        instr.push_str({
            match self.jump_if {
                JumpIf::Null => "000",
                JumpIf::GT => "001",
                JumpIf::EQ => "010",
                JumpIf::GE => "011",
                JumpIf::LT => "100",
                JumpIf::NE => "101",
                JumpIf::LE => "110",
                JumpIf::JMP => "111"
            }
        });
        instr
    }

}
impl Default for CInstr {
    fn default() -> Self {
        Self {
            line: 0,
            switch_a_for_m: false,
            comp: Comp::Zero,
            save_comp_to: SaveCompTo { a: false, d: false, m: false },
            jump_if: JumpIf::Null,
        }
    }
}

fn main() {
    let cmd_args = args().collect::<Vec<String>>();
    // 1st arg is always cwd, so we get the 2nd
    let path = Path::new(&cmd_args[1]);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    // let filename = path.to_str().unwrap().split('.').collect::<String>();
    // lol this is bonkers, you can turn the entire thing into a reference: &(blablabla)
    
    let instructions = lines_from_file(&path);
    // trim line & removee empty lines or comments
    let mut instructions = instructions.iter().filter(|line| {
        line.chars().count() > 0 && !line.starts_with("//")
    }).map(|line| line.trim()).collect::<Vec<_>>();

    // first parse
    let mut all_variables = HashMap::new();
    let mut line:u16 = 0;
    instructions.retain(|instr| {
        let mut chars = instr.chars();
        match chars.next().unwrap() {
            '@' => {
                let addr = chars.collect::<String>();
                match addr.parse::<u16>() {
                    Err(_) => {
                        if !all_variables.contains_key(&addr) {all_variables.insert(addr, line);};
                    },
                    _ => {}
                }
                line += 1;
                true                
            },
            '(' => {
                chars.next_back();
                let name = chars.collect::<String>();
                println!("{}", name);
                if let Some(value) = all_variables.get_mut(&name) {
                    *value = line;
                } else {
                    all_variables.insert(name, line);
                }
                false
            },
            _ => {
                // C instr or malfunctioning code
                line += 1;
                true
            }
        }

    });

    // second parse
    let mut path = String::from_str(filename).unwrap();
    path.push_str(".hack");
    let mut file = File::create(path).unwrap();
    for instr in instructions.iter() {
        let mut chars = instr.chars();
        let mut instr_as_binary = String::new();
        match chars.next().unwrap() {
            '@' => {
                instr_as_binary = String::from('0');
                let addr = chars.collect::<String>();
                let addr_as_u16 = addr.parse::<u16>();
                instr_as_binary.push_str(&format!("{:015b}",
                    match addr_as_u16 {
                    Err(_) => {
                        all_variables[&addr]
                    },
                    _ => addr_as_u16.unwrap()
                    }
                ));
                // println!("{} @ {}", instr_as_binary, addr);
            }
            '(' => {
                chars.next_back();
                let name = chars.collect::<String>();
                println!("{}", name);
                if let Some(value) = all_variables.get_mut(&name) {
                    *value = line;
                } else {
                    all_variables.insert(name, line);
                }
                continue;
            }
            first_char @ _ => {
                let has_save_to = instr.contains('=');
                let has_jump = instr.contains(';');
                // C instr or jump
                let mut c_instr = CInstr {..Default::default()};

                let mut first_part: String = chars.by_ref().take_while(|&x| !(x==';' || x=='=')).collect();
                first_part.insert(0, first_char);

                // this actually SKIPS ';', so we don't need to char.next() before defining jmp
                let second_part: String = chars.by_ref().take_while(|&x| x!=';').collect();

                let third_part: String = chars.collect();

                let mut save_to: Option<&str> = None;
                let mut comp: Option<&str> = None;
                let mut jump: Option<&str> = None;

                if has_save_to {
                    // AA=BB(;OPTIONAL)
                    save_to = Some(&first_part);
                    comp = Some(&second_part);
                    if has_jump {jump = Some(&third_part);};
                } else {
                    // BB(;OPTIONAL) (but if no jump then what tf is this instruction for)
                    comp = Some(&first_part);
                    jump = Some(&second_part);
                }

                println!("{} | {} | {}", save_to.unwrap_or_else(|| ""), comp.unwrap(), jump.unwrap_or_else(|| ""));

                match save_to {
                    Some(string) => {
                        if string.contains('A') {c_instr.save_comp_to.a = true;};
                        if string.contains('D') {c_instr.save_comp_to.d = true;};
                        if string.contains('M') {c_instr.save_comp_to.m = true;};
                    },
                    None => {},
                }

                match comp {
                    Some(string) => {
                        match string {
                            "0" => {c_instr.comp = Comp::Zero;} 
                            "1" => {c_instr.comp = Comp::One;} 
                            "-1" => {c_instr.comp = Comp::NegOne;} 
                            "D" => {c_instr.comp = Comp::D;} 
                            "A" => {c_instr.comp = Comp::X;} 
                            "M" => {c_instr.comp = Comp::X; c_instr.switch_a_for_m = true;} 
                            "!D" => {c_instr.comp = Comp::NotD;} 
                            "!A" => {c_instr.comp = Comp::NotX;} 
                            "!M" => {c_instr.comp = Comp::NotX; c_instr.switch_a_for_m = true;} 
                            "-D" => {c_instr.comp = Comp::NegD;} 
                            "-A" => {c_instr.comp = Comp::NegX;} 
                            "-M" => {c_instr.comp = Comp::NegX; c_instr.switch_a_for_m = true;} 
                            "D+1" => {c_instr.comp = Comp::DPlusOne;} 
                            "A+1" => {c_instr.comp = Comp::XPlusOne;} 
                            "M+1" => {c_instr.comp = Comp::XPlusOne; c_instr.switch_a_for_m = true;} 
                            "D-1" => {c_instr.comp = Comp::DMinusOne;} 
                            "A-1" => {c_instr.comp = Comp::XMinusOne;} 
                            "M-1" => {c_instr.comp = Comp::XMinusOne; c_instr.switch_a_for_m = true;} 
                            "D+A"|"A+D" => {c_instr.comp = Comp::SumDAndX;} 
                            "D+M"|"M+D" => {c_instr.comp = Comp::SumDAndX; c_instr.switch_a_for_m = true;} 
                            "D-A" => {c_instr.comp = Comp::DMinusX;} 
                            "D-M" => {c_instr.comp = Comp::DMinusX; c_instr.switch_a_for_m = true;} 
                            "A-D" => {c_instr.comp = Comp::XMinusD;} 
                            "M-D" => {c_instr.comp = Comp::XMinusD; c_instr.switch_a_for_m = true;} 
                            "D&A"|"A&D" => {c_instr.comp = Comp::DAndX;} 
                            "D&M"|"M&D" => {c_instr.comp = Comp::DAndX; c_instr.switch_a_for_m = true;} 
                            "D|A"|"A|D" => {c_instr.comp = Comp::DOrX;} 
                            "D|M"|"M|D" => {c_instr.comp = Comp::DOrX; c_instr.switch_a_for_m = true;} 
                            _ => {}
                        }
                    }
                    None => {}
                }

                match jump {
                    Some(string) => {
                        match string {
                            "JGT" => {c_instr.jump_if = JumpIf::GT;}
                            "JEQ" => {c_instr.jump_if = JumpIf::EQ;}
                            "JGE" => {c_instr.jump_if = JumpIf::GE;}
                            "JLT" => {c_instr.jump_if = JumpIf::LT;}
                            "JME" => {c_instr.jump_if = JumpIf::NE;}
                            "JLE" => {c_instr.jump_if = JumpIf::LE;}
                            "JMP" => {c_instr.jump_if = JumpIf::JMP;}
                            _ => {}
                        }
                    }
                    None => {}
                }

                println!("{} -> {:?}", c_instr.instr_as_binary(), c_instr);
                instr_as_binary = c_instr.instr_as_binary();

            }
        }
        
        line += 1;
        writeln!(file, "{}", instr_as_binary).unwrap();
    };


    let example = CInstr {
        line: 5, 
        switch_a_for_m: true,
        comp: Comp::SumDAndX,
        save_comp_to: SaveCompTo {a: true, ..Default::default()}, 
        jump_if: JumpIf::GE,
};

    println!("{}", example.instr_as_binary());
    for (k,v) in &all_variables {
        println!("{} -> {}", k, v);
    }
}
