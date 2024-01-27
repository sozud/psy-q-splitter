use rabbitizer::{config, Abi, InstrCategory, Instruction, OperandType};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandEnd {}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandCode {
    len: u16,
    bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandSectionSwitch {
    id: u16,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandUninitializedData {
    size: u32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct ExprConstant {
    value: u32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct ExprAddrOfSymbol {
    idx: u16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct ExprSectionBase {
    idx: u16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct ExprAdd {
    left: Option<Rc<Expr>>,
    right: Option<Rc<Expr>>,
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
enum Expr {
    Expr0(ExprConstant),
    Expr2(ExprAddrOfSymbol),
    Expr4(ExprSectionBase),
    Expr2C(ExprAdd),
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandReloc {
    type_: u8,
    offset: u16,
    expr: Expr,
}
#[derive(Serialize, Deserialize, PartialEq)]
struct CommandXdef {
    number: u16,
    section_id: u16,
    offset: u32,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandXref {
    number: u16,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandSection {
    section_id: u16,
    group: u8,
    align: u16,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandLocalSym {
    section_id: u16,
    offset: u32,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandFileName {
    number: u16,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandCpu {
    cpu: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct CommandXbss {
    number: u16,
    section_id: u16,
    size: u32,
    len: u8,
    name: String,
}

#[derive(Serialize, Deserialize)]
enum Command {
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

struct SerializedObj {
    commands: Vec<Command>,
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get8(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = file_contents[physical_address as usize + 0] as u32;
    return instr;
}

fn get16(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = (file_contents[physical_address as usize + 1] as u32) << 8
        | file_contents[physical_address as usize + 0] as u32;
    return instr;
}

fn get32(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = (file_contents[physical_address as usize + 3] as u32) << 24
        | (file_contents[physical_address as usize + 2] as u32) << 16
        | (file_contents[physical_address as usize + 1] as u32) << 8
        | file_contents[physical_address as usize + 0] as u32;
    return instr;
}

fn getn(source_vec: &Vec<u8>, start_pos: usize, n: usize) -> Vec<u8> {
    if start_pos < source_vec.len() {
        let sub_slice = &source_vec[start_pos..(start_pos + n).min(source_vec.len())];
        let destination_vec: Vec<u8> = sub_slice.into();
        destination_vec
    } else {
        Vec::new()
    }
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

struct Function {
    instructions: Vec<Instruction>,
    name: String,
}

struct CodeSection {
    len: u32,
    start_offset: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum RelocationTypes {
    Constant,
    AddressOfSymbol,
    SectionBase,
    Add,
}

use std::rc::Rc;

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

fn read_expression_recursive(
    file_contents: &Vec<u8>,
    offset: &mut usize,
    reloc_offset: usize,
    section: &mut Section,
) -> Option<Relocation> {
    let op = get8(file_contents, offset.clone());
    *offset += 1;

    println!("read_expression {:02X}", op);
    match op {
        0 => {
            // constant
            let value = get32(file_contents, offset.clone());
            let thing: Relocation = Relocation {
                value: Some(value),
                symbol_idx: None,
                section_idx: None,
                offset: reloc_offset,
                type_: RelocationTypes::Constant,
                left: None,
                right: None,
            };
            *offset += 4;
            return Some(thing);
        }
        2 => {
            // addr of symbol
            let idx = get16(file_contents, offset.clone());
            let thing = Relocation {
                symbol_idx: Some(idx),
                section_idx: None,
                value: None,
                offset: reloc_offset,
                type_: RelocationTypes::AddressOfSymbol,
                left: None,
                right: None,
            };
            *offset += 2;
            return Some(thing);
        }
        4 => {
            // section base
            let idx = get16(file_contents, offset.clone());
            let thing = Relocation {
                symbol_idx: None,
                section_idx: Some(idx),
                value: None,
                offset: reloc_offset,
                type_: RelocationTypes::SectionBase,
                left: None,
                right: None,
            };
            *offset += 2;
            return Some(thing);
        }
        0x2c => {
            // add
            let left = read_expression_recursive(file_contents, offset, reloc_offset, section);
            let right = read_expression_recursive(file_contents, offset, reloc_offset, section);

            if let (Some(left_value), Some(right_value)) = (left, right) {
                let thing = Relocation {
                    symbol_idx: None,
                    section_idx: None,
                    value: None,
                    offset: reloc_offset,
                    type_: RelocationTypes::Add,
                    left: Some(Rc::new(left_value.clone())),
                    right: Some(Rc::new(right_value.clone())),
                };

                return Some(thing);
            } else {
                println!("reloc fail");
                std::process::exit(1);
            }
        }
        _ => {
            println!("unknown op {:02X}", op);
            std::process::exit(1);
        }
    }
    None
}

fn read_expression_expr(
    file_contents: &Vec<u8>,
    offset: &mut usize,
    reloc_offset: usize,
    section: &mut Section,
) -> Expr {
    let op = get8(file_contents, offset.clone());
    *offset += 1;

    println!("read_expression {:02X}", op);
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
            let left = read_expression_expr(file_contents, offset, reloc_offset, section);
            let right = read_expression_expr(file_contents, offset, reloc_offset, section);

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

    assert!(false);
    Expr::Expr0(ExprConstant { value: 0 });
}

fn read_expression_serialize(
    file_contents: &Vec<u8>,
    offset: &mut usize,
    reloc_offset: usize,
    section: &mut Section,
    offset_v: u16,
) -> CommandReloc {
    let op = get8(file_contents, offset.clone());
    *offset += 1;

    println!("read_expression {:02X}", op);
    match op {
        0 => {
            // constant
            let value = get32(file_contents, offset.clone());
            *offset += 4;
            return CommandReloc {
                type_: op as u8,
                offset: offset_v as u16,
                expr: Expr::Expr0(ExprConstant { value: value }),
            };
        }
        2 => {
            // addr of symbol
            let idx = get16(file_contents, offset.clone());
            *offset += 2;

            return CommandReloc {
                type_: op as u8,
                offset: offset_v as u16,
                expr: Expr::Expr2(ExprAddrOfSymbol { idx: idx as u16 }),
            };
        }
        4 => {
            // section base
            let idx = get16(file_contents, offset.clone());
            *offset += 2;
            return CommandReloc {
                type_: op as u8,
                offset: offset_v as u16,
                expr: Expr::Expr4(ExprSectionBase { idx: idx as u16 }),
            };
        }
        0x2c => {
            // add
            let left = read_expression_expr(file_contents, offset, reloc_offset, section);
            let right = read_expression_expr(file_contents, offset, reloc_offset, section);

            return CommandReloc {
                type_: op as u8,
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

    assert!(false);
    // should never arrive here
    let ret = CommandReloc {
        type_: op as u8,
        offset: reloc_offset as u16,
        expr: Expr::Expr0(ExprConstant { value: 0 }),
    };
    ret
    // None
}

struct Section {
    symbols: Vec<Symbol>,
    name: String,
    bytes: Vec<u8>,
    relocations: Vec<Relocation>,
    zeroes: usize,
}

fn find_reloc(
    section: &Section,
    cur_offset: usize,
    instr: u32,
    sections: &HashMap<usize, Section>,
) -> Option<String> {
    let mut symbols_map: HashMap<u32, Symbol> = HashMap::new();
    let mut relocs_map: HashMap<usize, Relocation> = HashMap::new();
    for relocation in &section.relocations {
        relocs_map.insert(relocation.offset, relocation.clone());
    }

    for symbol in &section.symbols {
        if let Some(symbol_number) = symbol.number {
            symbols_map.insert(symbol_number, symbol.clone());
        }
    }

    if let Some(relocation) = relocs_map.get(&cur_offset) {
        if let Some(relocation_symbol_index) = relocation.symbol_idx {
            if let Some(symbol) = symbols_map.get(&relocation_symbol_index) {
                let instruction = Instruction::new(instr, 0, InstrCategory::CPU);
                let thing = instruction.disassemble(None, 0);
                println!(
                    "got symbol for reloc: cur_off {} rel_off {} name {} instr {} type {:?}",
                    cur_offset, relocation.offset, symbol.name, thing, relocation.type_
                );
                return Some(symbol.name.clone());
            }
        }
    }

    // generate add stuff.
    for relocation in &section.relocations {
        if !relocation.offset == cur_offset {
            continue;
        }
        if relocation.type_ == RelocationTypes::Add {
            if let (Some(left_value), Some(right_value)) =
                (relocation.left.clone(), relocation.right.clone())
            {
                if left_value.type_ == RelocationTypes::SectionBase
                    && right_value.type_ == RelocationTypes::Constant
                {
                    if let Some(section_idx) = left_value.section_idx {
                        if let Some(section_base_section) = sections.get(&(section_idx as usize)) {
                            if let Some(value) = right_value.value {
                                if section_base_section.name == ".data" {
                                    for s in &section_base_section.symbols {
                                        if let Some(the_offset) = s.offset {
                                            if value == the_offset {
                                                if right_value.offset == cur_offset {
                                                    return Some(s.name.clone());
                                                }
                                            }
                                        }
                                    }

                                    // value is offset into data section
                                    let name = format!("D_{:08X}", value);

                                    if right_value.offset == cur_offset {
                                        return Some(name);
                                    }
                                } else if section_base_section.name == ".text" {
                                    let name = format!(".L{:08X}", value);

                                    if right_value.offset == cur_offset {
                                        return Some(name);
                                    }
                                } else if section_base_section.name == ".bss" {
                                    // look for a bss symbol with this offset
                                    for s in &section_base_section.symbols {
                                        if let Some(the_offset) = s.offset {
                                            if value == the_offset {
                                                if right_value.offset == cur_offset {
                                                    return Some(s.name.clone());
                                                }
                                            }
                                        }
                                    }

                                    // value is offset into bss section
                                    let name = format!("B_{:08X}", value);

                                    if right_value.offset == cur_offset {
                                        return Some(name);
                                    }
                                } else if section_base_section.name == ".rdata" {
                                    // value is offset into rodata section?
                                    let name = format!("R_{:08X}", value);

                                    if right_value.offset == cur_offset {
                                        return Some(name);
                                    }
                                } else {
                                    println!(
                                        "missing reloc handling {}",
                                        section_base_section.name
                                    );
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                } else if left_value.type_ == RelocationTypes::Constant
                    && right_value.type_ == RelocationTypes::AddressOfSymbol
                {
                    // struct accesses work like this
                    if let Some(value) = left_value.value {
                        if let Some(idx) = right_value.symbol_idx {
                            if let Some(symbol) = symbols_map.get(&idx) {
                                let name = format!("{}+{}", symbol.name, value);

                                // either one has same offset
                                if right_value.offset == cur_offset {
                                    return Some(name);
                                }
                            }
                        }
                    }
                } else if left_value.type_ == RelocationTypes::Constant
                    && right_value.type_ == RelocationTypes::Add
                {
                    // Constant + (SectionBase + Constant)
                    if let (Some(right_left), Some(right_right)) =
                        (right_value.left.clone(), right_value.right.clone())
                    {
                        if right_left.type_ == RelocationTypes::SectionBase
                            && right_right.type_ == RelocationTypes::Constant
                        {
                            if let Some(right_left_section_idx) = right_left.section_idx {
                                if let Some(section_base_section) =
                                    sections.get(&(right_left_section_idx as usize))
                                {
                                    if let Some(right_right_value) = right_right.value {
                                        if let Some(left_value_value) = left_value.value {
                                            // all offsets seem to always be the same

                                            assert!(section_base_section.name == ".bss");

                                            for s in &section_base_section.symbols {
                                                if let Some(the_offset) = s.offset {
                                                    if right_right_value == the_offset {
                                                        let name = format!(
                                                            "{}+{}",
                                                            s.name, left_value_value
                                                        );

                                                        if right_value.offset == cur_offset {
                                                            return Some(name);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("unimplemented compound add reloc");
                            std::process::exit(1);
                        }
                    } else {
                        println!("something wrong with compound add reloc");
                        std::process::exit(1);
                    }
                } else {
                    println!(
                        "unhandled reloc {:?}:{:?} {:?}:{:?}\n",
                        left_value.type_, left_value.value, right_value.type_, right_value.value
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    None
}

use std::fs;
use std::io::Write;

struct Func {
    name: String,
    code: String,
}

struct Obj {
    name: String,
    funcs: Vec<Func>,
}

fn generate_branch_targets(
    file_contents: &Vec<u8>,
    cur_offset_: usize,
    start_offset: usize,
    code: &Vec<u8>,
    branch_target_source_map: &mut HashMap<usize, String>,
    branch_target_destination_map: &mut HashMap<usize, String>,
) {
    let mut cur_offset = cur_offset_;
    while cur_offset < start_offset + code.len() as usize {
        let symbol_addr = &(cur_offset - start_offset);
        let instr: u32 = get32(&file_contents, cur_offset);
        cur_offset += 4;
        let instruction = Instruction::new(instr, 0, InstrCategory::CPU);
        let thing = instruction.disassemble(None, 0);
        if instruction.is_branch() {
            println!(
                "symbol_addr {} instruction.branch_offset() {}",
                symbol_addr,
                instruction.branch_offset()
            );
            let addr = *symbol_addr as i32 + instruction.branch_offset();
            let target_label = format!(".L{:08X}", addr);
            println!(
                "branch_offset {} {} {} {}",
                thing.to_string(),
                instruction.branch_offset(),
                target_label,
                addr
            );

            assert!(addr != 0);
            assert!(*symbol_addr != 0);
            branch_target_destination_map.insert(addr as usize, target_label.clone());
            branch_target_source_map.insert(*symbol_addr, target_label);
        }
    }
}

fn do_code_section(
    cur_obj: &mut Obj,
    section: &Section,
    symbol_map: &HashMap<usize, Symbol>,
    sections: &HashMap<usize, Section>,
    aspsx_mode: bool,
) {
    let mut branch_target_destination_map: HashMap<usize, String> = HashMap::new();

    // relocs and branch targets need to have the same destination map
    // or we get L_XXXXXX twice in output

    // generate jump target relocs
    for relocation in &section.relocations {
        if relocation.type_ == RelocationTypes::Add {
            if let (Some(left_value), Some(right_value)) =
                (relocation.left.clone(), relocation.right.clone())
            {
                if left_value.type_ == RelocationTypes::SectionBase
                    && right_value.type_ == RelocationTypes::Constant
                {
                    if let Some(section_idx) = left_value.section_idx {
                        if let Some(section_base_section) = sections.get(&(section_idx as usize)) {
                            if section_base_section.name == ".text" {
                                if let Some(value) = right_value.value {
                                    let name = format!(".L{:08X}", value as usize);
                                    println!("generated {} {} {}", name, right_value.offset, value);

                                    // some sort of bug todo
                                    if value != 0 {
                                        branch_target_destination_map.insert(value as usize, name);
                                    } else {
                                        println!("branch target was 0");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let code = &section.bytes;
    {
        let start_offset = 0;
        let mut cur_offset = start_offset;
        let mut cur_func_string = "".to_string();
        let mut func_base = 0;

        let mut cur_func_name = "".to_string();

        println!("starting code section len {}", code.len());

        // need to get every non-reloc jump target to have labels. e.g.
        // blez $a2, .L8001FBB8 compared to
        // blez $a2, . + 4 + (0x25 << 2)

        let mut branch_target_source_map: HashMap<usize, String> = HashMap::new();

        generate_branch_targets(
            code,
            cur_offset,
            start_offset,
            code,
            &mut branch_target_source_map,
            &mut branch_target_destination_map,
        );

        while cur_offset < start_offset + code.len() as usize {
            let symbol_addr = &(cur_offset - start_offset);

            match symbol_map.get(symbol_addr) {
                Some(found_symbol) => {
                    println!("got symbol {} addr {}", found_symbol.name, symbol_addr);

                    cur_func_string +=
                        format!(".size {}, . - {}\n", cur_func_name, cur_func_name).as_str();

                    if cur_func_string.len() > 0 {
                        let cur_func = Func {
                            name: cur_func_name.clone(),
                            code: cur_func_string.clone(),
                        };

                        // emit the previous func
                        cur_obj.funcs.push(cur_func);
                    }

                    cur_func_string = "".to_string();

                    if aspsx_mode {
                        cur_func_string += ".set noat\r\n";
                        cur_func_string += ".set noreorder\r\n\r\n";
                        cur_func_string += format!(".globl {}\r\n", found_symbol.name).as_str();
                    } else {
                        cur_func_string += ".set noat      /* allow manual use of $at */\n";
                        cur_func_string +=
                            ".set noreorder /* don't insert nops after branches */\n\n";
                        cur_func_string += format!("glabel {}\n", found_symbol.name).as_str();
                    }

                    cur_func_name = found_symbol.name.clone();
                }
                None => {}
            }
            let instr: u32 = get32(&code, cur_offset);
            let instruction = Instruction::new(instr, 0, InstrCategory::CPU);

            match find_reloc(section, cur_offset - start_offset, instr, &sections) {
                Some(reloc) => {
                    if instruction.can_be_hi() {
                        let thing = instruction.disassemble(Some(&format!("%hi({})", reloc)), 0);
                        // check if this is also a target first and emit the label
                        match branch_target_destination_map.get(symbol_addr) {
                            Some(found_target_symbol) => {
                                let thing = instruction.disassemble(None, 0);
                                if aspsx_mode {
                                    cur_func_string +=
                                        format!("{}:\r\n", found_target_symbol).as_str();
                                } else {
                                    cur_func_string +=
                                        format!("{}:\n", found_target_symbol).as_str();
                                }
                            }
                            None => {}
                        }
                        if aspsx_mode {
                            cur_func_string += format!("{}\r\n", thing).as_str();
                        } else {
                            cur_func_string +=
                                format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing)
                                    .as_str();
                        }
                    } else if instruction.can_be_lo() {
                        let thing = instruction.disassemble(Some(&format!("%lo({})", reloc)), 0);
                        // check if this is also a target first and emit the label
                        match branch_target_destination_map.get(symbol_addr) {
                            Some(found_target_symbol) => {
                                let thing = instruction.disassemble(None, 0);
                                if aspsx_mode {
                                    cur_func_string +=
                                        format!("{}:\r\n", found_target_symbol).as_str();
                                } else {
                                    cur_func_string +=
                                        format!("{}:\n", found_target_symbol).as_str();
                                }
                            }
                            None => {}
                        }
                        if aspsx_mode {
                            cur_func_string += format!("{}\r\n", thing).as_str();
                        } else {
                            cur_func_string +=
                                format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing)
                                    .as_str();
                        }
                    } else {
                        let imm_override: Option<&str> = Some(&reloc);
                        let thing = instruction.disassemble(imm_override, 0);

                        // check if this is also a target first and emit the label
                        match branch_target_destination_map.get(symbol_addr) {
                            Some(found_target_symbol) => {
                                let thing = instruction.disassemble(None, 0);
                                if aspsx_mode {
                                    cur_func_string +=
                                        format!("{}:\r\n", found_target_symbol).as_str();
                                } else {
                                    cur_func_string +=
                                        format!("{}:\n", found_target_symbol).as_str();
                                }
                            }
                            None => {}
                        }

                        if aspsx_mode {
                            cur_func_string += format!("{}\r\n", thing).as_str();
                        } else {
                            cur_func_string +=
                                format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing)
                                    .as_str();
                        }
                    }
                }
                None => {
                    // check for non-reloc branch sources / targets
                    match branch_target_source_map.get(symbol_addr) {
                        Some(found_source_symbol) => {
                            // check if this is also a target first and emit the label
                            match branch_target_destination_map.get(symbol_addr) {
                                Some(found_target_symbol) => {
                                    let thing = instruction.disassemble(None, 0);

                                    if aspsx_mode {
                                        cur_func_string +=
                                            format!("{}:\r\n", found_target_symbol).as_str();
                                    } else {
                                        cur_func_string +=
                                            format!("{}:\n", found_target_symbol).as_str();
                                    }
                                }
                                None => {}
                            }

                            let imm_override: Option<&str> = Some(&found_source_symbol);
                            let thing = instruction.disassemble(imm_override, 0);

                            if aspsx_mode {
                                cur_func_string += format!("{}\r\n", thing).as_str();
                            } else {
                                cur_func_string +=
                                    format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing)
                                        .as_str();
                            }
                        }
                        None => {
                            match branch_target_destination_map.get(symbol_addr) {
                                Some(found_target_symbol) => {
                                    let thing = instruction.disassemble(None, 0);

                                    if aspsx_mode {
                                        cur_func_string +=
                                            format!("{}:\r\n", found_target_symbol).as_str();
                                        cur_func_string += format!("{}\r\n", thing).as_str();
                                    } else {
                                        cur_func_string +=
                                            format!("{}:\n", found_target_symbol).as_str();
                                        cur_func_string += format!(
                                            "/* {:08X} {:08X} */ {}\n",
                                            cur_offset, instr, thing
                                        )
                                        .as_str();
                                    }
                                }
                                None => {
                                    // vanilla instruction
                                    let thing = instruction.disassemble(None, 0);

                                    if aspsx_mode {
                                        cur_func_string += format!("{}\r\n", thing).as_str();
                                    } else {
                                        cur_func_string += format!(
                                            "/* {:08X} {:08X} */ {}\n",
                                            cur_offset, instr, thing
                                        )
                                        .as_str();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            cur_offset += 4;
        }

        if cur_func_string.len() > 0 {
            if aspsx_mode {
                cur_func_string += format!(".end {}\r\n", cur_func_name).as_str();
            } else {
                cur_func_string +=
                    format!(".size {}, . - {}\n", cur_func_name, cur_func_name).as_str();
            }

            let cur_func = Func {
                name: cur_func_name.clone(),
                code: cur_func_string.clone(),
            };

            println!("pushing {}", cur_func_name);

            cur_obj.funcs.push(cur_func);
        }
    }
}

fn disassemble_obj(
    sections: &HashMap<usize, Section>,
    name: String,
    file_contents: &Vec<u8>,
    output_path: &str,
    objs: &mut Vec<Obj>,
    aspsx_mode: bool,
) {
    let mut cur_obj = Obj {
        name: name.clone(),
        funcs: Vec::new(),
    };

    for (number, section) in sections.iter() {
        println!("~section name {}", section.name);

        let mut symbol_map: HashMap<usize, Symbol> = HashMap::new();

        // print relocs
        for relocation in &section.relocations {
            for symbol in &section.symbols {
                if let Some(number) = symbol.number {
                    if let Some(relocation_symbol_index) = relocation.symbol_idx {
                        if number == relocation_symbol_index {
                            println!(
                                "~~~~reloc number {} name {} offset {} type {:?}",
                                number, symbol.name, relocation.offset, relocation.type_
                            );
                        }
                    }
                }
            }
        }

        for symbol in &section.symbols {
            if let Some(offset) = symbol.offset {
                symbol_map.insert(offset as usize, symbol.clone());
            } else {
                println!("Offset is None");
            }
            println!("section {} symbol name {}", section.name, symbol.name);
        }

        if section.name == ".text" {
            do_code_section(&mut cur_obj, section, &symbol_map, sections, aspsx_mode);
        } else if section.name == ".data" {
            if section.bytes.len() > 0 {
                println!("data section");

                for (index, byte) in section.bytes.iter().enumerate() {
                    print!("{:02X} ", byte);
                    if (index + 1) % 16 == 0 {
                        println!();
                    }
                }
                println!();
                // std::process::exit(1);
            }
        } else if section.name == ".rdata" {
            if section.bytes.len() > 0 {
                println!("rdata section");

                for (index, byte) in section.bytes.iter().enumerate() {
                    print!("{:02X} ", byte);
                    if (index + 1) % 16 == 0 {
                        println!();
                    }
                }
                println!();
                // std::process::exit(1);
            }
        }

        let output_dir = format!("{}/{}", output_path, name).to_string();
        fs::create_dir_all(output_dir).unwrap();

        // Iterate over each Func and write to a file
        for func in &cur_obj.funcs {
            if func.name != "" {
                let filename = format!("{}/{}/{}.s", output_path, name, func.name);
                let mut file = fs::File::create(filename).unwrap();
                file.write_all(func.code.as_bytes()).unwrap();
            }
        }
    }

    objs.push(cur_obj);
}

fn parse_obj_inner(
    file_contents: &Vec<u8>,
    offset: usize,
    name: String,
    output_path: &str,
    objs: &mut Vec<Obj>,
    start_offset: usize,
    end_offset: &mut usize,
    commands: &mut Vec<Command>,
    aspsx_mode: bool,
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

                if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                    println!(
                        "code section cur_offset {} len {} file_contents len  {}",
                        cur_offset,
                        len,
                        file_contents.len()
                    );
                    if section.zeroes > 0 {
                        println!(
                            "extending section {} with {} 0s",
                            section.name, section.zeroes
                        );
                        section
                            .bytes
                            .extend(std::iter::repeat(0).take(section.zeroes));
                        section.zeroes = 0;
                    }
                    section
                        .bytes
                        .extend_from_slice(&file_contents[cur_offset..cur_offset + len as usize]);
                } else {
                    println!("missing section");
                    std::process::exit(0);
                }

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

                if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                    section.zeroes += size as usize;
                } else {
                    println!("missing section");
                    std::process::exit(0);
                }
            }
            10 => {
                let type_ = get8(&file_contents, cur_offset);
                cur_offset += 1;

                let offset = get16(&file_contents, cur_offset);
                cur_offset += 2;

                if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                    let temp = cur_offset;
                    let ser = read_expression_serialize(
                        &file_contents,
                        &mut cur_offset,
                        offset as usize + patch_offset,
                        section,
                        offset as u16,
                    );

                    commands.push(Command::Command10(ser));
                    cur_offset = temp;

                    let result = read_expression_recursive(
                        &file_contents,
                        &mut cur_offset,
                        offset as usize + patch_offset,
                        section,
                    );
                    if let Some(got_it) = result {
                        section.relocations.push(got_it);
                    }
                } else {
                    println!("missing section");
                    std::process::exit(0);
                }
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

                        let new_struct = Symbol {
                            number: Some(number),
                            section: section_id,
                            offset: Some(offset),
                            len: len,
                            name: string,
                            size: None,
                        };

                        if let Some(section) = sections.get_mut(&(section_id as usize)) {
                            section.symbols.push(new_struct);
                        } else {
                            println!("missing section");
                            std::process::exit(0);
                        }
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

                        let new_struct = Symbol {
                            number: Some(number),
                            section: cur_section_id,
                            offset: None,
                            len: len,
                            name: string,
                            size: None,
                        };

                        if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                            section.symbols.push(new_struct);
                        } else {
                            println!("missing section");
                            std::process::exit(0);
                        }
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

                        if let Some(section) = sections.get_mut(&(section_id as usize)) {
                            println!("section already exists");
                            std::process::exit(1);
                        } else {
                            let new_section = Section {
                                symbols: Vec::new(),
                                name: string,
                                bytes: Vec::new(),
                                relocations: Vec::new(),
                                zeroes: 0,
                            };
                            sections.insert(section_id as usize, new_section);
                        }
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

                        if let Some(section) = sections.get_mut(&(section_id as usize)) {
                            let new_struct = Symbol {
                                number: None,
                                section: section_id,
                                offset: Some(offset),
                                len: len,
                                name: string,
                                size: None,
                            };

                            section.symbols.push(new_struct);
                        } else {
                            println!("section doesnt exist");
                            std::process::exit(1);
                        }
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
                println!("cpu {}", cpu);
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

                        if let Some(section) = sections.get_mut(&(section_id as usize)) {
                            let new_struct = Symbol {
                                number: Some(number),
                                section: section_id,
                                offset: None,
                                size: Some(size),
                                len: len,
                                name: string,
                            };

                            section.symbols.push(new_struct);
                        } else {
                            println!("section doesnt exist");
                            std::process::exit(1);
                        }
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

    disassemble_obj(
        &sections,
        name,
        file_contents,
        &output_path,
        objs,
        aspsx_mode,
    );
}

fn parse_obj(
    file_contents: &Vec<u8>,
    offset: usize,
    name: String,
    output_path: &str,
    objs: &mut Vec<Obj>,
    aspsx_mode: bool,
) -> usize {
    let magic_offset: usize = offset as usize + 0;
    let magic: u32 = get32(&file_contents, magic_offset);
    let mut end_offset: usize = 0;

    if magic == 0x024B4E4C
    //OBJ
    {
        let mut commands: Vec<Command> = Vec::new();

        parse_obj_inner(
            file_contents,
            offset,
            name,
            output_path,
            objs,
            magic_offset + 4,
            &mut end_offset,
            &mut commands,
            aspsx_mode,
        );
    } else {
        println!("not an obj  {:08X} \n", magic);
    }

    end_offset
}

use std::env;

fn parse_lib(
    input_path: &str,
    output_path: &str,
    objs: &mut Vec<Obj>,
    target_obj_name: &Option<String>,
    aspsx_mode: bool,
) {
    let mut current_pos = 0;

    match read_file_to_vec(input_path) {
        Ok(file_contents) => {
            let thing = get32(&file_contents, 0);
            let mut base_addr: usize = 4;
            current_pos += 4;

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

                            if let Some(obj_name) = target_obj_name {
                                if string.trim() == obj_name {
                                    parse_obj(
                                        &file_contents,
                                        offset as usize + base_addr as usize,
                                        lowercase_name,
                                        output_path,
                                        objs,
                                        aspsx_mode,
                                    );
                                }
                            } else {
                                parse_obj(
                                    &file_contents,
                                    offset as usize + base_addr as usize,
                                    lowercase_name,
                                    output_path,
                                    objs,
                                    aspsx_mode,
                                );
                            }
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
                let lowercase_name = "output".to_string();
                let mut end_offset = 0;
                let mut commands: Vec<Command> = Vec::new();
                parse_obj_inner(
                    &file_contents,
                    offset,
                    lowercase_name,
                    output_path,
                    objs,
                    4,
                    &mut end_offset,
                    &mut commands,
                    aspsx_mode,
                );
            }
        }
        Err(error) => {
            println!("Error: {:?} {}", error, input_path);
            std::process::exit(1);
        }
    }
}

// ignore different file names
fn ignore_file_name_difference(a: &Command, b: &Command) -> bool {
    if let (Command::Command28(a_cmd), Command::Command28(b_cmd)) = (a, b) {
        if a_cmd.name != b_cmd.name {
            println!("skipping file name mismatch {} {}", a_cmd.name, b_cmd.name);
            return true;
        }
        return true;
    } else {
        false
    }
}

fn diff_objs(expected_path: String, actual_path: String) {
    let mut expected_commands: Vec<Command> = Vec::new();
    let output_path = "../output";

    println!("comparing {} and {}", expected_path, actual_path);

    match read_file_to_vec(&expected_path) {
        Ok(expected_contents) => {
            let thing = get32(&expected_contents, 0);

            if thing == 0x024B4E4C
            // LNK
            {
                let offset = 0;
                let lowercase_name = "output".to_string();
                let mut end_offset = 0;
                let mut objs: Vec<Obj> = Vec::new();

                parse_obj_inner(
                    &expected_contents,
                    offset,
                    lowercase_name,
                    output_path,
                    &mut objs,
                    4,
                    &mut end_offset,
                    &mut expected_commands,
                    false,
                );
            }
        }
        Err(error) => {
            println!("Error: {:?} expected_path {}", error, expected_path);
            std::process::exit(1);
        }
    }

    let mut actual_commands: Vec<Command> = Vec::new();

    match read_file_to_vec(&actual_path) {
        Ok(actual_contents) => {
            let thing = get32(&actual_contents, 0);

            if thing == 0x024B4E4C
            // LNK
            {
                let offset = 0;
                let lowercase_name = "output".to_string();
                let mut end_offset = 0;
                let mut objs: Vec<Obj> = Vec::new();
                parse_obj_inner(
                    &actual_contents,
                    offset,
                    lowercase_name,
                    output_path,
                    &mut objs,
                    4,
                    &mut end_offset,
                    &mut actual_commands,
                    false,
                );
            }
        }
        Err(error) => {
            println!("Error: {:?} actual_path {}", error, actual_path);
            std::process::exit(1);
        }
    }

    let header = format!("{: <32}{}", "expected", "actual");
    println!("{}", header);

    for (command_e, command_a) in expected_commands.iter().zip(actual_commands.iter()) {
        let spacing = 20;

        let e_string = serde_json::to_string_pretty(&command_e).unwrap();
        let a_string = serde_json::to_string_pretty(&command_a).unwrap();

        let expected_lines: Vec<&str> = e_string.lines().collect();
        let actual_lines: Vec<&str> = a_string.lines().collect();

        let max_len1 = expected_lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);

        // Iterate through the lines and print them side by side
        for (line1, line2) in expected_lines.iter().zip(actual_lines.iter()) {
            let padded_string = format!("{: <32}{}", line1, line2);
            println!("{}", padded_string);
        }

        if command_e != command_a {
            if ignore_file_name_difference(command_e, command_a) {
                continue;
            }
            println!("mismatch");
            std::process::exit(1);
        }
    }

    println!("objs matched");
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let aspsx_mode = false;

    if aspsx_mode {
        unsafe {
            config::RabbitizerConfig_Cfg.reg_names.named_registers = false;
            config::RabbitizerConfig_Cfg.toolchain_tweaks.sn64_div_fix = true;
        }
    }

    if args.len() < 3 {
        eprintln!("Usage: {} <input_path> <output_path>", args[0]);
        std::process::exit(1);
    }

    if args[1] == "diff" {
        diff_objs(args[2].clone(), args[3].clone());
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let mut objs: Vec<Obj> = Vec::new();

    parse_lib(input_path, output_path, &mut objs, &None, aspsx_mode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use similar::{ChangeTag, TextDiff};

    fn print_diff(expected_lines: String, actual_lines: String) {
        let diff = TextDiff::from_lines(&expected_lines, &actual_lines);

        for diff in diff.iter_all_changes() {
            match diff.tag() {
                ChangeTag::Delete => print!("\x1b[31m{}\x1b[0m", diff),
                ChangeTag::Insert => print!("\x1b[32m{}\x1b[0m", diff),
                ChangeTag::Equal => print!("{}", diff),
            }
        }
        println!();
    }

    use regex::Regex;

    fn trim_comments(input: &str) -> String {
        // replace comments
        let comment_regex = Regex::new(r"/\*.*?\*/").unwrap();
        let result_1 = comment_regex.replace_all(input, "");

        // replace spaces
        let whitespace_pattern = Regex::new(r" +").unwrap();
        let result_2 = whitespace_pattern.replace_all(&result_1, " ");

        // remove addresses from jumps
        let pattern = Regex::new(r"\.L[0-9a-fA-F]{8}").unwrap();
        let replacement = ".L";
        let result_3 = pattern.replace_all(&result_2, replacement);

        result_3.to_string()
    }

    fn check_func(objs: &Vec<Obj>, expected: &str, name: &str) {
        for obj in objs {
            for func in &obj.funcs {
                if func.name == name {
                    let expected_trimmed = trim_comments(&expected);
                    let actual_trimmed = trim_comments(&func.code);

                    let expected_clone = expected_trimmed.clone();
                    let actual_clone = actual_trimmed.clone();

                    print_diff(expected_trimmed, actual_trimmed);

                    let expected_lines: Vec<&str> = expected_clone.lines().collect();
                    let actual_lines: Vec<&str> = actual_clone.lines().collect();

                    let max_len1 = expected_lines
                        .iter()
                        .map(|line| line.len())
                        .max()
                        .unwrap_or(0);

                    // Iterate through the lines and print them side by side
                    for (line1, line2) in expected_lines.iter().zip(actual_lines.iter()) {
                        if line1 != line2 {
                            let spacing = max_len1.saturating_sub(line1.len());
                            println!("{}{}{}", line1, " ".repeat(spacing), line2);
                            assert_eq!(line1, line2);
                        }
                    }
                }
            }
        }
    }

    fn compare_asm(file_path: &str, name: &str, obj_name: &Option<String>) {
        let input_path = "../psy-q/PSX/LIB/LIBSND.LIB";
        let output_path = "../output_directory";
        let mut objs: Vec<Obj> = Vec::new();

        parse_lib(input_path, output_path, &mut objs, obj_name, false);

        match fs::read_to_string(file_path) {
            Ok(contents) => {
                check_func(&objs, &contents, name);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    }

    #[test]
    fn test_SsUtResolveADSR() {
        compare_asm(
            "test_data/_SsUtResolveADSR.s",
            "_SsUtResolveADSR",
            &Some("ADSR".to_string()),
        );
    }

    #[test]
    fn test_SsSndCrescendo() {
        compare_asm(
            "test_data/_SsSndCrescendo.s",
            "_SsSndCrescendo",
            &Some("CRES".to_string()),
        );
    }

    #[test]
    fn testSpuVmAlloc() {
        compare_asm(
            "test_data/SpuVmAlloc.s",
            "SpuVmAlloc",
            &Some("VMANAGER".to_string()),
        );
    }

    #[test]
    fn testSpuVmVSetUp() {
        compare_asm(
            "test_data/SpuVmVSetUp.s",
            "SpuVmVSetUp",
            &Some("VM_VSU".to_string()),
        );
    }

    #[test]
    fn testSsVabTransBodyPartly() {
        compare_asm(
            "test_data/SsVabTransBodyPartly.s",
            "SsVabTransBodyPartly",
            &Some("VS_VTBP".to_string()),
        );
    }

    #[test]
    fn test_SsInitSoundSep() {
        compare_asm(
            "test_data/_SsInitSoundSep.s",
            "_SsInitSoundSep",
            &Some("SEPINIT".to_string()),
        );
    }
}
