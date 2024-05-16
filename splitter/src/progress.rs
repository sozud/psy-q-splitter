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
pub struct ObjProgress {
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
    json_data_bytes: usize,
    json_data_done: usize,
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
            json_data_bytes: 0,
            json_data_done: 0,
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

fn print_progress(progress_vec: &Vec<ObjProgress>) -> ObjProgress {
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
        json_data_bytes: 0,
        json_data_done: 0,
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

    sum.json_data_bytes = sum.bss_bytes + sum.data_bytes + sum.rdata_bytes;
    sum.json_data_done = sum.bss_done + sum.data_done + sum.rdata_done;

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

    sum
}

pub fn do_progress(lib_path: &String, build_path: &String) -> ObjProgress {
    let default = ObjProgress {
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
        json_data_bytes: 0,
        json_data_done: 0,
    };

    match read_file_to_vec(lib_path) {
        Ok(file_contents) => {
            let lib = serialize_parse_lib(&file_contents);
            let result = calculate_progress(
                &lib,
                &file_contents,
                &lib_path.to_string(),
                &build_path.to_string(),
            );
            return print_progress(&result);
        }
        Err(error) => {
            println!("Error: {:?} lib_path {}", error, lib_path);
            std::process::exit(1);
        }
    }
    default
}

use git2::{Repository, Time};
use reqwest;
use reqwest::Client;
use serde_json::json;
use serde_json::to_string_pretty;
use std::env;

pub fn send_json() -> Result<(), reqwest::Error> {
    let mut git_hash = String::new();
    let mut git_timestamp = 0;

    if let Ok(repo) = Repository::open("../../..") {
        // Get the HEAD reference
        if let Ok(head) = repo.head() {
            if let Some(head_oid) = head.target() {
                // Convert the OID to a string
                if let hash = head_oid.to_string() {
                    println!("Git hash: {}", hash);
                    git_hash = hash.clone();
                } else {
                    println!("Failed to convert OID to string.");
                }
            } else {
                println!("Failed to get target of HEAD reference.");
            }

            // Resolve the reference to a commit
            if let Ok(commit) = head.peel_to_commit() {
                // Get the commit's timestamp
                let time = commit.time();

                // Convert the timestamp to a human-readable string
                let timestamp = time.seconds();
                git_timestamp = timestamp;

                println!("Commit timestamp: {}", timestamp);
            } else {
                println!("Failed to resolve HEAD reference to commit.");
            }
        } else {
            println!("Failed to get HEAD reference.");
        }
    } else {
        println!("Failed to open repository.");
    }

    let lib_paths = vec!["../psy-q/PSX/LIB/LIBSND.LIB", "../psy-q/PSX/LIB/LIBSPU.LIB"];
    let build_paths = vec!["../../../build/snd", "../../../build/spu"];
    let slugs = vec!["snd", "spu"];

    let mut code_info = serde_json::Map::new();
    let mut code_inner = serde_json::Map::new();
    let mut data_inner = serde_json::Map::new();

    code_info.insert("timestamp".into(), git_timestamp.into());
    code_info.insert("git_hash".into(), git_hash.clone().into());

    for pos in 0..lib_paths.len() {
        let lib_path = &lib_paths[pos];
        let build_path = &build_paths[pos];
        let slug = &slugs[pos];

        let prog = do_progress(&lib_path.to_string(), &build_path.to_string());

        // code_info.insert(
        //     // slug.to_string(),
        //     json!({
        //         &format!("{}", slug): prog.text_done,
        //         &format!("{}/total", slug): prog.text_bytes,
        //     }),
        // );
        code_inner.insert(format!("{}_code", slug).into(), prog.text_done.into());
        code_inner.insert(
            format!("{}_code/total", slug).into(),
            prog.text_bytes.into(),
        );

        code_inner.insert(format!("{}_data", slug).into(), prog.json_data_done.into());
        code_inner.insert(
            format!("{}_data/total", slug).into(),
            prog.json_data_bytes.into(),
        );

        // data_info.insert(
        //     slug.to_string(),
        //     json!({
        //         &format!("{}", slug): prog.json_data_done,
        //         &format!("{}/total", slug): prog.json_data_bytes,
        //     }),
        // );
    }

    // let code_wrapped = serde_json::json!({"code": serde_json::Value::Object(code_inner)});
    // let data_wrapped = serde_json::json!({"data": serde_json::Value::Object(data_inner)});

    code_info.insert("measures".into(), serde_json::Value::Object(code_inner));

    let mut outer = serde_json::Map::new();

    outer.insert(
        "api_key".into(),
        env::var("API_SECRET").expect("API_SECRET not set").into(),
    );

    let entries = vec![code_info.clone()];

    outer.insert("entries".into(), entries.into());

    // code_info.insert(
    //     "measures".into(),
    //     serde_json::Value::Object(data-wrapped)
    // );

    //     let mut measures = serde_json::Map::new();
    // measures.extend(code_info);

    // let json_data = json!({
    //     "timestamp": git_timestamp,
    //     "git_hash": git_hash.clone(),
    //     "measures": {
    //         @for (key, value) in &code_info {
    //             // Convert key-value pairs to JSON format
    //             (key): (value),
    //         }
    //     }
    // });
    if let Ok(json_string) = serde_json::to_string(&outer) {
        if let Ok(pretty_json) = to_string_pretty(&outer) {
            println!("json_data:\n{}", pretty_json);
        } else {
            println!("Failed to pretty print JSON data.");
        }
    }

    // std::process::exit(0);

    // println!("json_data: {:?}", json_data);

    // std::process::exit(0);
    let api_base_url = env::var("API_BASE_URL").expect("API_BASE_URL not set");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not set");

    let client = reqwest::blocking::Client::new();
    let url = format!("{}/data/psyq/3.5", api_base_url);

    if let Ok(json_string) = serde_json::to_string(&outer) {
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("api_key", &api_secret)
            .body(json_string)
            .send()?;
        println!("Response: {:?}", response);
    }

    Ok(())
}

// fn main() {
//     if let Err(err) = send_json() {
//         eprintln!("Error: {}", err);
//     }
// }

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
