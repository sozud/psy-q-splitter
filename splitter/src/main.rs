use rabbitizer::{config, Abi, InstrCategory, Instruction, OperandType};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

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

struct Section {
    symbols: Vec<Symbol>,
    name: String,
    bytes: Vec<CodeSection>,
    relocations: Vec<Relocation>,
}

fn find_reloc(section: &Section, cur_offset: usize, instr: u32) -> Option<String> {
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
                    "got symbol for reloc: cur_off {} rel_off {} name {} instr {}",
                    cur_offset, relocation.offset, symbol.name, thing
                );
                return Some(symbol.name.clone());
            }
        }
    }

    // generate add stuff.
    for relocation in &section.relocations {
        if relocation.type_ == RelocationTypes::Add {
            if let (Some(left_value), Some(right_value)) =
                (relocation.left.clone(), relocation.right.clone())
            {
                if left_value.type_ == RelocationTypes::SectionBase
                    && right_value.type_ == RelocationTypes::Constant
                {
                    if let Some(value) = right_value.value {
                        let name = format!(".L_{:08X}", value);
                        // probably ought to look up section?

                        if right_value.offset == cur_offset {
                            return Some(name);
                        }
                    }
                }
            }
        }
    }

    None
}

use std::fs;
use std::io::Write;

fn disassemble_obj(sections: &HashMap<usize, Section>, name: String, file_contents: &Vec<u8>, output_path: &String) {
    // disassemble

    struct Func {
        name: String,
        code: String,
    }

    let mut funcs: Vec<Func> = Vec::new();

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
            println!("symbol name {}", symbol.name);
        }

        let mut jump_target_map: HashMap<usize, String> = HashMap::new();

        // generate jump target relocs
        for relocation in &section.relocations {
            if relocation.type_ == RelocationTypes::Add {
                if let (Some(left_value), Some(right_value)) =
                    (relocation.left.clone(), relocation.right.clone())
                {
                    if left_value.type_ == RelocationTypes::SectionBase
                        && right_value.type_ == RelocationTypes::Constant
                    {
                        if let Some(value) = right_value.value {
                            let name = format!(".L_{:08X}", value as usize);
                            println!("generated {} {} {}", name, right_value.offset, value);
                            jump_target_map.insert(value as usize, name);
                        }
                    }
                }
            }
        }

        for code in &section.bytes {
            let start_offset = code.start_offset;
            let mut cur_offset = start_offset;
            let mut cur_func_string = "".to_string();
            let mut func_base = 0;

            let mut cur_func_name = "".to_string();

            println!("starting code section");

            while cur_offset < start_offset + code.len as usize {
                let symbol_addr = &(cur_offset - start_offset);

                match jump_target_map.get(symbol_addr) {
                    Some(found_symbol) => {
                        println!("got jump target {}", found_symbol);
                        cur_func_string += format!("{}\n", found_symbol).as_str();
                    }
                    None => {}
                }

                match symbol_map.get(symbol_addr) {
                    Some(found_symbol) => {
                        println!("got symbol {}", found_symbol.name);

                        if cur_func_string.len() > 0 {
                            let cur_func = Func {
                                name: cur_func_name.clone(),
                                code: cur_func_string.clone(),
                            };

                            // emit the previous func
                            funcs.push(cur_func);
                        }

                        cur_func_string = "".to_string();

                        cur_func_string += format!("glabel {}\n", found_symbol.name).as_str();

                        cur_func_name = found_symbol.name.clone();
                    }
                    None => {}
                }
                let instr: u32 = get32(&file_contents, cur_offset);
                cur_offset += 4;
                let instruction = Instruction::new(instr, 0, InstrCategory::CPU);

                // why off by 4?
                match find_reloc(section, (cur_offset - start_offset) - 4, instr) {
                    Some(reloc) => {
                        let imm_override: Option<&str> = Some(&reloc);
                        let thing = instruction.disassemble(imm_override, 0);
                        cur_func_string +=
                            format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing).as_str();
                    }
                    None => {
                        let thing = instruction.disassemble(None, 0);
                        cur_func_string +=
                            format!("/* {:08X} {:08X} */ {}\n", cur_offset, instr, thing).as_str();
                    }
                }
            }

            if cur_func_string.len() > 0 {
                let cur_func = Func {
                    name: cur_func_name.clone(),
                    code: cur_func_string.clone(),
                };

                funcs.push(cur_func);
            }
        }

        let output_dir = format!("{}/{}", output_path, name).to_string();
        fs::create_dir_all(output_dir).unwrap();

        // Iterate over each Func and write to a file
        for func in &funcs {
            let filename = format!("{}/{}/{}.s", output_path, name, func.name);
            let mut file = fs::File::create(filename).unwrap();
            file.write_all(func.code.as_bytes()).unwrap();
        }
    }
}

fn parse_obj(file_contents: &Vec<u8>, offset: usize, name: String, output_path: &String) -> usize {
    let magic_offset: usize = offset as usize + 0;
    let magic: u32 = get32(&file_contents, magic_offset);
    let mut end_offset: usize = 0;
    let mut sections: HashMap<usize, Section> = HashMap::new();
    let mut cur_section_id = 0;

    if magic == 0x024B4E4C
    //OBJ
    {
        let mut functions: Vec<Function> = Vec::new();
        let mut cur_offset = magic_offset + 4;
        loop {
            let chunk = get8(&file_contents, cur_offset);
            cur_offset += 1;
            println!("chunk {}", chunk);

            match chunk {
                0 => {
                    // end
                    break;
                }
                2 => {
                    // code section
                    let len: u32 = get16(&file_contents, cur_offset);
                    cur_offset += 2;
                    println!("code size {}", len);

                    let code_section: CodeSection = CodeSection {
                        len: len,
                        start_offset: cur_offset,
                    };

                    if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                        section.bytes.push(code_section);
                    } else {
                        println!("missing section");
                        std::process::exit(0);
                    }

                    cur_offset += len as usize;
                }
                6 => {
                    // section switch
                    let id = get16(&file_contents, cur_offset);
                    cur_section_id = id;
                    cur_offset += 2;
                }
                8 => {
                    //	Uninitialised data
                    let size = get32(&file_contents, cur_offset);
                    cur_offset += 4;
                }
                10 => {
                    let type_ = get8(&file_contents, cur_offset);
                    cur_offset += 1;

                    let offset = get16(&file_contents, cur_offset);
                    cur_offset += 2;

                    if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                        let result = read_expression_recursive(
                            &file_contents,
                            &mut cur_offset,
                            offset as usize,
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

                            if let Some(section) = sections.get_mut(&(section_id as usize)) {
                                println!("section already exists");
                                std::process::exit(1);
                            } else {
                                let new_section = Section {
                                    symbols: Vec::new(),
                                    name: string,
                                    bytes: Vec::new(),
                                    relocations: Vec::new(),
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
                            println!("local sym name {} len {}", string, len);

                            if let Some(section) = sections.get_mut(&(section_id as usize)) {
                                let new_struct = Symbol {
                                    number: None, // fixme
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
                            println!("file name {} number {}", string, number);
                        }
                        Err(_) => todo!(),
                    }
                }
                46 => {
                    let cpu = get8(&file_contents, cur_offset);
                    cur_offset += 1;
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
            end_offset = cur_offset;
        }

        disassemble_obj(&sections, name, file_contents, &output_path);
    } else {
        println!("not an obj  {:08X} \n", magic);
    }

    end_offset
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_path> <output_path>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

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

                            parse_obj(
                                &file_contents,
                                offset as usize + base_addr as usize,
                                lowercase_name,
                                output_path
                            );
                            base_addr += size as usize;
                            current_pos = base_addr;
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        Err(error) => {
            println!("Error: {:?} {}", error, input_path);
            std::process::exit(1);
        }
    }
}
