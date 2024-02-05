// pub mod file_io;

use std::fs::File;
use std::io::Read;
use std::io::{self};

pub fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn get8(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = file_contents[physical_address as usize + 0] as u32;
    return instr;
}

pub fn get16(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = (file_contents[physical_address as usize + 1] as u32) << 8
        | file_contents[physical_address as usize + 0] as u32;
    return instr;
}

pub fn get32(file_contents: &Vec<u8>, physical_address: usize) -> u32 {
    let instr: u32 = (file_contents[physical_address as usize + 3] as u32) << 24
        | (file_contents[physical_address as usize + 2] as u32) << 16
        | (file_contents[physical_address as usize + 1] as u32) << 8
        | file_contents[physical_address as usize + 0] as u32;
    return instr;
}

pub fn getn(source_vec: &Vec<u8>, start_pos: usize, n: usize) -> Vec<u8> {
    if start_pos < source_vec.len() {
        let sub_slice = &source_vec[start_pos..(start_pos + n).min(source_vec.len())];
        let destination_vec: Vec<u8> = sub_slice.into();
        destination_vec
    } else {
        Vec::new()
    }
}

pub fn set8(file_contents: &mut Vec<u8>, physical_address: usize, value: u8) {
    file_contents.resize(physical_address + 1, 0);
    if physical_address < file_contents.len() {
        file_contents[physical_address] = value;
    }
}

pub fn set16(file_contents: &mut Vec<u8>, physical_address: usize, value: u16) {
    file_contents.resize(physical_address + 2, 0);
    if physical_address + 1 < file_contents.len() {
        file_contents[physical_address] = (value & 0xFF) as u8;
        file_contents[physical_address + 1] = ((value >> 8) & 0xFF) as u8;
    }
}

pub fn set32(file_contents: &mut Vec<u8>, physical_address: usize, value: u32) {
    file_contents.resize(physical_address + 4, 0);
    if physical_address + 3 < file_contents.len() {
        file_contents[physical_address] = (value & 0xFF) as u8;
        file_contents[physical_address + 1] = ((value >> 8) & 0xFF) as u8;
        file_contents[physical_address + 2] = ((value >> 16) & 0xFF) as u8;
        file_contents[physical_address + 3] = ((value >> 24) & 0xFF) as u8;
    }
}

pub fn setn(file_contents: &mut Vec<u8>, start_pos: usize, values: &[u8]) {
    let end_pos = start_pos + values.len();
    file_contents.resize(end_pos, 0);
    if end_pos <= file_contents.len() {
        file_contents[start_pos..end_pos].copy_from_slice(values);
    }
}
