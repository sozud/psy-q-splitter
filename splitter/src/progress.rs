use crate::file_io::{get16, get32, get8, getn, read_file_to_vec, set16, set32, set8, setn};
use crate::get_obj_from_lib;
use crate::objs_are_mismatched;
use crate::serialize_parse_lib;
use crate::{Command, Section, SerializedLib, SerializedObj, Symbol};
use rabbitizer::Instruction;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct ObjProgress {
    name: String,
    done: bool,
    functions_done: usize,
    functions_total: usize,
    text_bytes: usize,
    text_done: usize,
    bss_bytes: usize,
    bss_done: usize,
    data_bytes: usize,
    data_done: usize,
    rdata_bytes: usize,
    rdata_done: usize,
}

fn calculate_progress(
    serialized_lib: &SerializedLib,
    expected: &Vec<u8>,
    lib_path: &String,
    build_dir: &String,
) -> Vec<ObjProgress> {
    let mut progress_vec: Vec<ObjProgress> = Vec::new();

    for serialized_lib_obj in &serialized_lib.objs {
        let mut sections: HashMap<usize, Section> = HashMap::new();
        let mut cur_section_id = 0;

        for command in &serialized_lib_obj.obj.commands {
            match command {
                Command::Command0(cmd) => {
                    break;
                }
                Command::Command2(cmd) => {
                    // bytes
                    if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
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
                        section.bytes.extend_from_slice(&cmd.bytes);
                    } else {
                        println!("missing section {}", cur_section_id);
                        std::process::exit(0);
                    }
                }
                Command::Command6(cmd) => {
                    // section switch
                    cur_section_id = cmd.id;
                }
                Command::Command8(cmd) => {
                    if let Some(section) = sections.get_mut(&(cur_section_id as usize)) {
                        section.zeroes += cmd.size as usize;
                    } else {
                        println!("missing section");
                        std::process::exit(0);
                    }
                }
                Command::Command10(cmd) => {}
                Command::Command12(cmd) => {
                    // xdef
                    let new_struct = Symbol {
                        number: Some(cmd.number as u32),
                        section: cmd.section_id as u32,
                        offset: Some(cmd.offset),
                        len: cmd.len as u32,
                        name: cmd.name.clone(),
                        size: None,
                    };

                    if let Some(section) = sections.get_mut(&(cmd.section_id as usize)) {
                        section.symbols.push(new_struct);
                    } else {
                        println!("missing section");
                        std::process::exit(0);
                    }
                }
                Command::Command14(cmd) => {}
                Command::Command16(cmd) => {
                    // section
                    if let Some(section) = sections.get_mut(&(cmd.section_id as usize)) {
                        println!("section already exists");
                        std::process::exit(1);
                    } else {
                        let new_section = Section {
                            symbols: Vec::new(),
                            name: cmd.name.clone(),
                            bytes: Vec::new(),
                            relocations: Vec::new(),
                            zeroes: 0,
                        };
                        sections.insert(cmd.section_id as usize, new_section);
                    }
                }
                Command::Command18(cmd) => {
                    if let Some(section) = sections.get_mut(&(cmd.section_id as usize)) {
                        let new_struct = Symbol {
                            number: None,
                            section: cmd.section_id as u32,
                            offset: Some(cmd.offset as u32),
                            len: cmd.len as u32,
                            name: cmd.name.clone(),
                            size: None,
                        };

                        section.symbols.push(new_struct);
                    } else {
                        println!("section doesnt exist");
                        std::process::exit(1);
                    }
                }
                Command::Command28(cmd) => {}
                Command::Command46(cmd) => {}
                Command::Command48(cmd) => {}
                _ => todo!(),
            }
        }

        let mut cur_obj = ObjProgress {
            name: serialized_lib_obj.name.clone(),
            done: false,
            functions_total: 0,
            functions_done: 0,
            text_bytes: 0,
            text_done: 0,
            bss_bytes: 0,
            bss_done: 0,
            data_bytes: 0,
            data_done: 0,
            rdata_bytes: 0,
            rdata_done: 0,
        };

        for (id, section) in &sections {
            if section.name == ".text" {
                cur_obj.text_bytes += section.bytes.len();

                cur_obj.functions_total += section.symbols.len();
            }
            if section.name == ".data" {
                cur_obj.data_bytes += section.bytes.len();
            }
            if section.name == ".bss" {
                cur_obj.bss_bytes += section.bytes.len();

                // not sure about this
                cur_obj.bss_bytes += section.zeroes;
            }
            if section.name == ".rdata" {
                cur_obj.rdata_bytes += section.bytes.len();
            }
        }

        // here we need to look for the compiled obj, check if it matches, and increase
        if let Ok(entries) = fs::read_dir(build_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();

                    if let Some(file_str) = file_name.to_str() {
                        let formatted_name = serialized_lib_obj.name.trim().to_lowercase();
                        let new_file_name = format!("{}.obj", formatted_name);

                        if new_file_name == file_str {
                            // found an obj, compare it.
                            if let Some(expected_contents) =
                                get_obj_from_lib(lib_path, &serialized_lib_obj.name)
                            {
                                match read_file_to_vec(&entry.path().to_string_lossy()) {
                                    Ok(actual_contents) => {
                                        if !objs_are_mismatched(
                                            &expected_contents,
                                            &actual_contents,
                                        ) {
                                            cur_obj.done = true;
                                            cur_obj.text_done = cur_obj.text_bytes;
                                            cur_obj.functions_done = cur_obj.functions_total;
                                            cur_obj.data_done = cur_obj.data_bytes;
                                            cur_obj.bss_done = cur_obj.bss_bytes;
                                            cur_obj.rdata_done = cur_obj.rdata_bytes;
                                        }
                                    }
                                    Err(error) => {
                                        println!(
                                            "Error: {:?} actual_path {}",
                                            error,
                                            entry.path().to_string_lossy()
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            } else {
                                println!("mismatch error");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory {}", build_dir);
            std::process::exit(1);
        }

        progress_vec.push(cur_obj.clone());
    }
    progress_vec
}

fn print_progress(progress_vec: &Vec<ObjProgress>) {
    let mut sum = ObjProgress {
        name: String::new(),
        done: false,
        functions_done: 0,
        functions_total: 0,
        text_bytes: 0,
        text_done: 0,
        bss_bytes: 0,
        bss_done: 0,
        data_bytes: 0,
        data_done: 0,
        rdata_bytes: 0,
        rdata_done: 0,
    };

    let mut objs_done = 0;

    for obj in progress_vec {
        println!("{}", obj.name);
        if obj.functions_total > 0 {
            println!(
                "  funcs : {}/{} ({:.2}%)",
                obj.functions_done,
                obj.functions_total,
                (obj.functions_done as f64 / obj.functions_total as f64) * 100.0
            );
        }
        if obj.text_bytes > 0 {
            println!(
                "  .text : {}/{} ({:.2}%)",
                obj.text_done,
                obj.text_bytes,
                (obj.text_done as f64 / obj.text_bytes as f64) * 100.0
            );
        }
        if obj.data_bytes > 0 {
            println!(
                "  .data : {}/{} ({:.2}%)",
                obj.data_done,
                obj.data_bytes,
                (obj.data_done as f64 / obj.data_bytes as f64) * 100.0
            );
        }
        if obj.bss_bytes > 0 {
            println!(
                "  .bss  : {}/{} ({:.2}%)",
                obj.bss_done,
                obj.bss_bytes,
                (obj.bss_done as f64 / obj.bss_bytes as f64) * 100.0
            );
        }
        if obj.rdata_bytes > 0 {
            println!(
                "  .rdata: {}/{} ({:.2}%)",
                obj.rdata_done,
                obj.rdata_bytes,
                (obj.rdata_done as f64 / obj.rdata_bytes as f64) * 100.0
            );
        }
        println!("");

        if obj.done {
            objs_done += 1;
        }

        sum.functions_done += obj.functions_done;
        sum.functions_total += obj.functions_total;
        sum.text_bytes += obj.text_bytes;
        sum.text_done += obj.text_done;
        sum.bss_bytes += obj.bss_bytes;
        sum.bss_done += obj.bss_done;
        sum.data_bytes += obj.data_bytes;
        sum.data_done += obj.data_done;
        sum.rdata_bytes += obj.rdata_bytes;
        sum.rdata_done += obj.rdata_done;
    }

    println!("Total");

    if progress_vec.len() > 0 {
        println!(
            "  objs  : {}/{} ({:.2}%)",
            objs_done,
            progress_vec.len(),
            (objs_done as f64 / progress_vec.len() as f64) * 100.0
        );
    }

    if sum.functions_total > 0 {
        println!(
            "  funcs : {}/{} ({:.2}%)",
            sum.functions_done,
            sum.functions_total,
            (sum.functions_done as f64 / sum.functions_total as f64) * 100.0
        );
    }

    if sum.text_bytes > 0 {
        println!(
            "  .text : {}/{} ({:.2}%)",
            sum.text_done,
            sum.text_bytes,
            (sum.text_done as f64 / sum.text_bytes as f64) * 100.0
        );
    }

    if sum.data_bytes > 0 {
        println!(
            "  .data : {}/{} ({:.2}%)",
            sum.data_done,
            sum.data_bytes,
            (sum.data_done as f64 / sum.data_bytes as f64) * 100.0
        );
    }

    if sum.bss_bytes > 0 {
        println!(
            "  .bss  : {}/{} ({:.2}%)",
            sum.bss_done,
            sum.bss_bytes,
            (sum.bss_done as f64 / sum.bss_bytes as f64) * 100.0
        );
    }

    if sum.rdata_bytes > 0 {
        println!(
            "  .rdata: {}/{} ({:.2}%)",
            sum.rdata_done,
            sum.rdata_bytes,
            (sum.rdata_done as f64 / sum.rdata_bytes as f64) * 100.0
        );
    }
}

pub fn do_progress(lib_path: &String, build_path: &String) {
    match read_file_to_vec(lib_path) {
        Ok(file_contents) => {
            let lib = serialize_parse_lib(&file_contents);
            let result = calculate_progress(
                &lib,
                &file_contents,
                &lib_path.to_string(),
                &build_path.to_string(),
            );
            print_progress(&result);
        }
        Err(error) => {
            println!("Error: {:?} lib_path {}", error, lib_path);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_progress() {
    //     let input_path = "../psy-q/PSX/LIB/LIBSND.LIB";
    //     let build_path = "../../../build";
    //     do_progress(&input_path.to_string(), &build_path.to_string());
    // }
}
