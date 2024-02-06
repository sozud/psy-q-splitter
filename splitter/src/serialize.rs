use crate::file_io::{get16, get32, get8, getn, read_file_to_vec, set16, set32, set8, setn};
use rabbitizer::Instruction;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum RelocationTypes {
    Constant,
    AddressOfSymbol,
    SectionBase,
    Add,
}

#[derive(Clone)]
struct Relocation {
    symbol_idx: Option<u32>,
    section_idx: Option<u32>,
    value: Option<u32>,
    offset: usize,
    type_: RelocationTypes,
    left: Option<Rc<Relocation>>,
    right: Option<Rc<Relocation>>,
}

struct Function {
    instructions: Vec<Instruction>,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandEnd {}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandCode {
    pub len: u16,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandSectionSwitch {
    pub id: u16,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandUninitializedData {
    pub size: u32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ExprConstant {
    pub value: u32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ExprAddrOfSymbol {
    pub idx: u16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ExprSectionBase {
    pub idx: u16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ExprAdd {
    pub left: Option<Rc<Expr>>,
    pub right: Option<Rc<Expr>>,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Expr0(lhs), Expr::Expr0(rhs)) => lhs == rhs,
            (Expr::Expr2(lhs), Expr::Expr2(rhs)) => lhs == rhs,
            (Expr::Expr4(lhs), Expr::Expr4(rhs)) => lhs == rhs,
            (Expr::Expr2C(lhs), Expr::Expr2C(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Command::Command0(lhs), Command::Command0(rhs)) => lhs == rhs,
            (Command::Command2(lhs), Command::Command2(rhs)) => lhs == rhs,
            (Command::Command6(lhs), Command::Command6(rhs)) => lhs == rhs,
            (Command::Command8(lhs), Command::Command8(rhs)) => lhs == rhs,
            (Command::Command10(lhs), Command::Command10(rhs)) => lhs == rhs,
            (Command::Command12(lhs), Command::Command12(rhs)) => lhs == rhs,
            (Command::Command14(lhs), Command::Command14(rhs)) => lhs == rhs,
            (Command::Command16(lhs), Command::Command16(rhs)) => lhs == rhs,
            (Command::Command18(lhs), Command::Command18(rhs)) => lhs == rhs,
            (Command::Command28(lhs), Command::Command28(rhs)) => lhs == rhs,
            (Command::Command46(lhs), Command::Command46(rhs)) => lhs == rhs,
            (Command::Command48(lhs), Command::Command48(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Eq for Command {}

#[derive(Clone, Serialize, Deserialize)]
pub enum Expr {
    Expr0(ExprConstant),
    Expr2(ExprAddrOfSymbol),
    Expr4(ExprSectionBase),
    Expr2C(ExprAdd),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandReloc {
    pub type_: u8,
    pub offset: u16,
    pub expr: Expr,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandXdef {
    pub number: u16,
    pub section_id: u16,
    pub offset: u32,
    pub len: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandXref {
    pub number: u16,
    pub len: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandSection {
    pub section_id: u16,
    pub group: u8,
    pub align: u16,
    pub len: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandLocalSym {
    pub section_id: u16,
    pub offset: u32,
    pub len: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandFileName {
    pub number: u16,
    pub len: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandCpu {
    pub cpu: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CommandXbss {
    pub number: u16,
    pub section_id: u16,
    pub size: u32,
    pub len: u8,
    pub name: String,
}

#[derive(Clone)]
struct Symbol {
    number: Option<u32>,
    section: u32,
    offset: Option<u32>,
    len: u32,
    name: String,
    size: Option<u32>,
}

struct Section {
    symbols: Vec<Symbol>,
    name: String,
    bytes: Vec<u8>,
    relocations: Vec<Relocation>,
    zeroes: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Command0(CommandEnd),
    Command2(CommandCode),
    Command6(CommandSectionSwitch),
    Command8(CommandUninitializedData),
    Command10(CommandReloc),
    Command12(CommandXdef),
    Command14(CommandXref),
    Command16(CommandSection),
    Command18(CommandLocalSym),
    Command28(CommandFileName),
    Command46(CommandCpu),
    Command48(CommandXbss),
}

pub struct SerializedObj {
    magic: u32,
    pub commands: Vec<Command>,
}

pub struct SerializedLib {
    pub magic: u32,
    pub objs: Vec<SerializedLibObj>,
}

fn read_expression_expr(file_contents: &Vec<u8>, offset: &mut usize, reloc_offset: usize) -> Expr {
    let op = get8(file_contents, offset.clone());
    *offset += 1;

    match op {
        0 => {
            // constant
            let value = get32(file_contents, offset.clone());
            *offset += 4;
            return Expr::Expr0(ExprConstant { value: value });
        }
        2 => {
            // addr of symbol
            let idx = get16(file_contents, offset.clone());
            *offset += 2;
            return Expr::Expr2(ExprAddrOfSymbol { idx: idx as u16 });
        }
        4 => {
            // section base
            let idx = get16(file_contents, offset.clone());
            *offset += 2;
            return Expr::Expr4(ExprSectionBase { idx: idx as u16 });
        }
        0x2c => {
            // add
            let left = read_expression_expr(file_contents, offset, reloc_offset);
            let right = read_expression_expr(file_contents, offset, reloc_offset);

            return Expr::Expr2C(ExprAdd {
                left: Some(Rc::new(left.clone())),
                right: Some(Rc::new(right.clone())),
            });
        }
        _ => {
            println!("unknown op {:02X}", op);
            std::process::exit(1);
        }
    }
}

fn read_expression_serialize(
    file_contents: &Vec<u8>,
    offset: &mut usize,
    reloc_offset: usize,
    offset_v: u16,
    type_: u8,
) -> CommandReloc {
    let op = get8(file_contents, offset.clone());
    *offset += 1;

    match op {
        0 => {
            // constant
            let value = get32(file_contents, offset.clone());
            *offset += 4;
            return CommandReloc {
                type_: type_ as u8,
                offset: offset_v as u16,
                expr: Expr::Expr0(ExprConstant { value: value }),
            };
        }
        2 => {
            // addr of symbol
            let idx = get16(file_contents, offset.clone());
            *offset += 2;

            return CommandReloc {
                type_: type_ as u8,
                offset: offset_v as u16,
                expr: Expr::Expr2(ExprAddrOfSymbol { idx: idx as u16 }),
            };
        }
        4 => {
            // section base
            let idx = get16(file_contents, offset.clone());
            *offset += 2;
            return CommandReloc {
                type_: type_ as u8,
                offset: offset_v as u16,
                expr: Expr::Expr4(ExprSectionBase { idx: idx as u16 }),
            };
        }
        0x2c => {
            // add
            let left = read_expression_expr(file_contents, offset, reloc_offset);
            let right = read_expression_expr(file_contents, offset, reloc_offset);

            return CommandReloc {
                type_: type_ as u8,
                offset: offset_v as u16,
                expr: Expr::Expr2C(ExprAdd {
                    left: Some(Rc::new(left.clone())),
                    right: Some(Rc::new(right.clone())),
                }),
            };
        }
        _ => {
            println!("unknown op {:02X}", op);
            std::process::exit(1);
        }
    }
}

fn write_expression(bytes: &mut Vec<u8>, pos: &mut usize, expr_: &Expr, expected: &Vec<u8>) {
    match expr_ {
        Expr::Expr0(expr) => {
            set8(bytes, *pos, 0);
            *pos += 1;
            set32(bytes, *pos, expr.value);
            *pos += 4;
        }
        Expr::Expr2(expr) => {
            set8(bytes, *pos, 2);
            *pos += 1;
            set16(bytes, *pos, expr.idx);
            *pos += 2;
        }
        Expr::Expr4(expr) => {
            set8(bytes, *pos, 4);
            *pos += 1;
            set16(bytes, *pos, expr.idx);
            *pos += 2;
        }
        Expr::Expr2C(expr) => {
            set8(bytes, *pos, 0x2C);
            *pos += 1;
            if let Some(left) = &expr.left {
                write_expression(bytes, pos, &left, expected);
            }
            if let Some(right) = &expr.right {
                write_expression(bytes, pos, &right, expected);
            }
        }
        _ => todo!(),
    }
}

pub struct SerializedLibObj {
    pub name: String,
    pub date: u32,
    pub offset: u32,
    pub size: u32,
    pub obj: SerializedObj,
}

pub fn serialize_parse_lib(file_contents: &Vec<u8>) -> SerializedLib {
    let mut current_pos = 0;
    let mut serialized_lib = SerializedLib {
        magic: 0,
        objs: Vec::new(),
    };

    let thing = get32(&file_contents, 0);
    let mut base_addr: usize = 4;
    current_pos += 4;

    serialized_lib.magic = thing;

    if thing == 0x0142494C
    //LIB
    {
        loop {
            if current_pos + 8 > file_contents.len() {
                println!("end of file");
                break;
            }
            let byte_vec = getn(&file_contents, current_pos, 8);

            current_pos += 8;
            match String::from_utf8(byte_vec) {
                Ok(string) => {
                    let date: u32 = get32(&file_contents, current_pos);
                    current_pos += 4;

                    let offset: u32 = get32(&file_contents, current_pos);
                    current_pos += 4;

                    let size: u32 = get32(&file_contents, current_pos);
                    current_pos += 4;

                    println!(
                        "parsing {}.OBJ {:08X} offset {} base_off {} plus {}",
                        string.trim(),
                        date,
                        offset,
                        base_addr,
                        offset as usize + base_addr as usize
                    );

                    let lowercase_name = string.to_lowercase();

                    let obj = parse_obj(&file_contents, offset as usize + base_addr as usize);

                    serialized_lib.objs.push(SerializedLibObj {
                        name: string,
                        date: date,
                        offset: offset,
                        size: size,
                        obj: obj,
                    });
                    base_addr += size as usize;
                    current_pos = base_addr;
                }
                Err(err) => {
                    println!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    } else if thing == 0x024B4E4C
    // LNK
    {
        let offset = 0;
        let mut end_offset = 0;
        let mut commands: Vec<Command> = Vec::new();
        parse_obj_inner(&file_contents, offset, 4, &mut end_offset, &mut commands);
    }

    return serialized_lib;
}

fn parse_obj_inner(
    file_contents: &Vec<u8>,
    offset: usize,
    start_offset: usize,
    end_offset: &mut usize,
    commands: &mut Vec<Command>,
) {
    let mut sections: HashMap<usize, Section> = HashMap::new();
    let mut cur_section_id = 0;
    let mut patch_offset = 0;

    let mut functions: Vec<Function> = Vec::new();
    let mut cur_offset = start_offset;
    loop {
        let chunk = get8(&file_contents, cur_offset);
        cur_offset += 1;
        println!("chunk {}", chunk);

        match chunk {
            0 => {
                commands.push(Command::Command0(CommandEnd {}));
                // end
                break;
            }
            2 => {
                // code section
                let len: u32 = get16(&file_contents, cur_offset);
                cur_offset += 2;
                println!("code size {}", len);

                commands.push(Command::Command2(CommandCode {
                    len: len as u16,
                    bytes: file_contents[cur_offset..cur_offset + len as usize].to_vec(),
                }));

                cur_offset += len as usize;
            }
            6 => {
                // section switch
                let id = get16(&file_contents, cur_offset);

                commands.push(Command::Command6(CommandSectionSwitch { id: id as u16 }));

                cur_section_id = id;
                cur_offset += 2;
                if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                    patch_offset = section.bytes.len();
                }
            }
            8 => {
                //	Uninitialised data
                let size = get32(&file_contents, cur_offset);
                cur_offset += 4;

                commands.push(Command::Command8(CommandUninitializedData { size: size }));
            }
            10 => {
                let type_ = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let offset = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let ser = read_expression_serialize(
                    &file_contents,
                    &mut cur_offset,
                    offset as usize + patch_offset,
                    offset as u16,
                    type_ as u8,
                );

                commands.push(Command::Command10(ser));
            }
            12 => {
                let number = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let section_id = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let offset = get32(&file_contents, cur_offset);
                cur_offset += 4;

                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        println!("xdef name {} offset {}", string, offset);

                        commands.push(Command::Command12(CommandXdef {
                            number: number as u16,
                            section_id: section_id as u16,
                            offset: offset as u32,
                            len: len as u8,
                            name: string.clone(),
                        }));
                    }
                    Err(_) => todo!(),
                }
            }
            14 => {
                let number = get16(&file_contents, cur_offset);
                cur_offset += 2;
                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        println!("xref name {} len {} number {}", string, len, number);

                        commands.push(Command::Command14(CommandXref {
                            number: number as u16,
                            len: len as u8,
                            name: string.clone(),
                        }));
                    }
                    Err(_) => todo!(),
                }
            }
            16 => {
                // section
                let section_id = get16(&file_contents, cur_offset);
                cur_offset += 2;
                let group: u32 = get8(&file_contents, cur_offset);
                cur_offset += 1;
                let align: u32 = get16(&file_contents, cur_offset);
                cur_offset += 2;
                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        println!(
                            "section name {} id {} group {} align {} len {}",
                            string, section_id, group, align, len
                        );

                        commands.push(Command::Command16(CommandSection {
                            section_id: section_id as u16,
                            group: group as u8,
                            align: align as u16,
                            len: len as u8,
                            name: string.clone(),
                        }));
                    }
                    Err(_) => todo!(),
                }
            }
            18 => {
                let section_id = get16(&file_contents, cur_offset);
                cur_offset += 2;
                let offset: u32 = get32(&file_contents, cur_offset);
                cur_offset += 4;
                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        commands.push(Command::Command18(CommandLocalSym {
                            section_id: section_id as u16,
                            offset: offset as u32,
                            len: len as u8,
                            name: string.clone(),
                        }));

                        println!("local sym name {} offset {} len {}", string, offset, len);
                    }
                    Err(_) => todo!(),
                }
            }
            28 => {
                let number = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        commands.push(Command::Command28(CommandFileName {
                            number: number as u16,
                            len: len as u8,
                            name: string.clone(),
                        }));

                        println!("file name {} number {}", string, number);
                    }
                    Err(_) => todo!(),
                }
            }
            46 => {
                let cpu = get8(&file_contents, cur_offset);
                cur_offset += 1;

                commands.push(Command::Command46(CommandCpu { cpu: cpu as u8 }));
            }
            48 => {
                // xbss
                let number = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let section_id = get16(&file_contents, cur_offset);
                cur_offset += 2;

                let size = get32(&file_contents, cur_offset);
                cur_offset += 4;

                let len = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let name_vec: Vec<u8> = getn(&file_contents, cur_offset, len as usize);
                cur_offset += len as usize;
                match String::from_utf8(name_vec) {
                    Ok(string) => {
                        commands.push(Command::Command48(CommandXbss {
                            number: number as u16,
                            section_id: section_id as u16,
                            size: size as u32,
                            len: len as u8,
                            name: string.clone(),
                        }));

                        println!("xbss name {} number {}", string, number);
                    }
                    Err(_) => todo!(),
                }
            }
            _ => {
                println!("unknown chunk {}", chunk);
                std::process::exit(0);
            }
        }
        *end_offset = cur_offset;
    }
}

fn parse_obj(file_contents: &Vec<u8>, offset: usize) -> SerializedObj {
    let magic_offset: usize = offset as usize + 0;
    let magic: u32 = get32(&file_contents, magic_offset);

    let mut end_offset: usize = 0;

    let mut serialized_obj = SerializedObj {
        magic: magic,
        commands: Vec::new(),
    };

    if magic == 0x024B4E4C
    //OBJ
    {
        parse_obj_inner(
            file_contents,
            offset,
            magic_offset + 4,
            &mut end_offset,
            &mut serialized_obj.commands,
        );

        return serialized_obj;
    } else {
        println!("not an obj  {:08X} \n", magic);
        std::process::exit(1);
    }
}

fn serialize_lib(serialized_lib: &SerializedLib, bytes: &mut Vec<u8>, expected: &Vec<u8>) {
    let mut base_addr: usize = 4;
    let mut pos = 0;
    set32(bytes, 0, serialized_lib.magic);
    pos += 4;

    for serialized_lib_obj in &serialized_lib.objs {
        setn(bytes, pos, serialized_lib_obj.name.as_bytes());
        pos += 8;

        set32(bytes, pos, serialized_lib_obj.date);
        pos += 4;
        set32(bytes, pos, serialized_lib_obj.offset);
        pos += 4;

        set32(bytes, pos, serialized_lib_obj.size);
        pos += 4;

        let ascii_string = String::from_utf8_lossy(&[0x73, 0x53, 0x5F, 0x0E]);

        pos = serialized_lib_obj.offset as usize + base_addr;

        set32(bytes, pos, serialized_lib_obj.obj.magic);

        pos += 4;

        for command in &serialized_lib_obj.obj.commands {
            match command {
                Command::Command0(cmd) => {
                    set8(bytes, pos, 0);
                    break;
                }
                Command::Command2(cmd) => {
                    set8(bytes, pos, 2);
                    pos += 1;

                    set16(bytes, pos, cmd.len);
                    pos += 2;

                    setn(bytes, pos, &cmd.bytes);

                    pos += cmd.bytes.len();
                }
                Command::Command6(cmd) => {
                    set8(bytes, pos, 6);
                    pos += 1;
                    set16(bytes, pos, cmd.id);
                    pos += 2;
                }
                Command::Command8(cmd) => {
                    set8(bytes, pos, 8);
                    pos += 1;
                    set32(bytes, pos, cmd.size);
                    pos += 4;
                }
                Command::Command10(cmd) => {
                    set8(bytes, pos, 10);

                    pos += 1;

                    set8(bytes, pos, cmd.type_);

                    pos += 1;

                    set16(bytes, pos, cmd.offset);
                    pos += 2;

                    write_expression(bytes, &mut pos, &cmd.expr, &expected);
                }
                Command::Command12(cmd) => {
                    set8(bytes, pos, 12);
                    pos += 1;

                    set16(bytes, pos, cmd.number);
                    pos += 2;

                    set16(bytes, pos, cmd.section_id);
                    pos += 2;

                    set32(bytes, pos, cmd.offset);
                    pos += 4;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                Command::Command14(cmd) => {
                    set8(bytes, pos, 14);
                    pos += 1;

                    set16(bytes, pos, cmd.number);
                    pos += 2;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                Command::Command16(cmd) => {
                    set8(bytes, pos, 16);
                    pos += 1;

                    set16(bytes, pos, cmd.section_id);
                    pos += 2;

                    set8(bytes, pos, cmd.group);
                    pos += 1;

                    set16(bytes, pos, cmd.align);
                    pos += 2;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                Command::Command18(cmd) => {
                    set8(bytes, pos, 18);
                    pos += 1;

                    set16(bytes, pos, cmd.section_id);
                    pos += 2;

                    set32(bytes, pos, cmd.offset);
                    pos += 4;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                Command::Command28(cmd) => {
                    set8(bytes, pos, 28);
                    pos += 1;

                    set16(bytes, pos, cmd.number);
                    pos += 2;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                Command::Command46(cmd) => {
                    set8(bytes, pos, 46);
                    pos += 1;

                    set8(bytes, pos, cmd.cpu);
                    pos += 1;
                }
                Command::Command48(cmd) => {
                    set8(bytes, pos, 48);

                    pos += 1;

                    set16(bytes, pos, cmd.number);

                    pos += 2;

                    set16(bytes, pos, cmd.section_id);

                    pos += 2;

                    set32(bytes, pos, cmd.size);
                    pos += 4;

                    set8(bytes, pos, cmd.len);
                    pos += 1;

                    setn(bytes, pos, &cmd.name.as_bytes());
                    pos += cmd.name.len();
                }
                _ => todo!(),
            }

            // for (index, &item) in bytes.iter().enumerate() {
            //     let b = expected.get(index).unwrap();

            //     if(index < serialized_lib_obj.offset as usize + base_addr)
            //     {
            //         continue;
            //     }
            //     if (item != *b) {
            //         println!("bytes: {:08X} expected: {:08X}\n", get8(&bytes, index),  get8(&expected, index));
            //         println!("Mismatch at position {}", index);
            //         std::process::exit(1);
            //     }
            // }
        }

        base_addr += serialized_lib_obj.size as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get8() {
        let file_contents = vec![0x12, 0x34, 0x56, 0x78];
        assert_eq!(get8(&file_contents, 0), 0x12);
        assert_eq!(get8(&file_contents, 2), 0x56);
    }

    #[test]
    fn test_get16() {
        let file_contents = vec![0x12, 0x34, 0x56, 0x78];
        assert_eq!(get16(&file_contents, 0), 0x3412);
        assert_eq!(get16(&file_contents, 2), 0x7856);
    }

    #[test]
    fn test_get32() {
        let file_contents = vec![0x12, 0x34, 0x56, 0x78, 0x90];
        assert_eq!(get32(&file_contents, 0), 0x78563412);
        assert_eq!(get32(&file_contents, 1), 0x90785634);
    }

    #[test]
    fn test_set8() {
        let mut file_contents = vec![0, 0, 0, 0];
        set8(&mut file_contents, 0, 0x12);
        set8(&mut file_contents, 2, 0x56);
        assert_eq!(get8(&file_contents, 0), 0x12);
        assert_eq!(get8(&file_contents, 2), 0x56);
    }

    #[test]
    fn test_set16() {
        let mut file_contents = vec![0, 0, 0, 0];
        set16(&mut file_contents, 0, 0x3412);
        set16(&mut file_contents, 2, 0x7856);
        assert_eq!(get16(&file_contents, 0), 0x3412);
        assert_eq!(get16(&file_contents, 2), 0x7856);
    }

    #[test]
    fn test_set32() {
        let mut file_contents = vec![0, 0, 0, 0, 0];
        set32(&mut file_contents, 0, 0x78563412);
        set32(&mut file_contents, 1, 0x90785634);
        assert_eq!(get32(&file_contents, 0), 0x78563412);
        assert_eq!(get32(&file_contents, 1), 0x90785634);
    }

    #[test]
    fn test_getn() {
        let file_contents = vec![0x12, 0x34, 0x56, 0x78];
        assert_eq!(getn(&file_contents, 1, 2), vec![0x34, 0x56]);
        assert_eq!(getn(&file_contents, 2, 3), vec![0x56, 0x78]);
        assert_eq!(getn(&file_contents, 0, 5), vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_setn() {
        let mut file_contents = vec![0, 0, 0, 0];
        setn(&mut file_contents, 0, &vec![0x12, 0x34, 0x56, 0x78]);
        assert_eq!(getn(&file_contents, 0, 5), vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_serialize_lib() {
        let input_path = "../psy-q/PSX/LIB/LIBSND.LIB";
        let output_path = "../output_directory";
        match read_file_to_vec(input_path) {
            Ok(file_contents) => {
                let lib = serialize_parse_lib(&file_contents);
                let mut bytes: Vec<u8> = Vec::new();
                serialize_lib(&lib, &mut bytes, &file_contents);
                // assert!(file_contents.iter().eq(bytes.iter()));
                // for (index, &item) in file_contents.iter().enumerate() {
                //     let b = bytes.get(index).unwrap();
                //     assert_eq!(item, *b, "Mismatch at position {}", index);
                // }
            }
            Err(error) => {
                println!("Error: {:?} {}", error, input_path);
                std::process::exit(1);
            }
        }
    }
}
