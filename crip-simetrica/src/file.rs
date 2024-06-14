use std::fs;
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write, Read};

use crate::round_key::round_key_generate;
use crate::blocks_converter::blocks_converter;
use crate::crypt::crypt_blocks;
use crate::dcrypt::dcrypt_blocks;

pub fn text_encoding(init_file: &str, dest_file: &str, key: &[[u8; 4]; 4]) -> bool {
    let arq_init_lines = read_file_as_string(init_file).expect("");

    let keys_round = round_key_generate(*key);

    let blocks = blocks_converter(&arq_init_lines);
    let blocks_cript = crypt_blocks(&blocks, &keys_round);
    write_byte_matrix_to_file(&blocks_cript, dest_file).expect("");

    false
}

pub fn text_decoding(init_file: &str, dest_file: &str, key: &[[u8; 4]; 4]) -> bool {
    let blocks = read_matrices_from_file(init_file).expect("");

    let keys_round = round_key_generate(*key);

    let blocks_dcript = dcrypt_blocks(&blocks, &keys_round);
    let text = convert_matrices_to_string_array(&blocks_dcript);
    append_strings_to_file(&text, dest_file).expect("");

    false
}

fn convert_matrices_to_string_array(matrices: &[[[u8; 4]; 4]]) -> Vec<String> {
    let mut result = Vec::with_capacity(matrices.len());

    for matrix in matrices.iter() {
        let mut sb = String::new();
        for row in matrix.iter() {
            for &byte in row.iter() {
                sb.push(char::from(byte));
            }
        }
        result.push(sb);
    }

    result
}

pub fn delete_file_if_exists(file_path: &str) {
    if fs::metadata(file_path).is_ok() {
        let _ = fs::remove_file(file_path); 
    } else {
        println!("O arquivo não existe: {}", file_path);
    }
}

fn append_strings_to_file(data: &Vec<String>, file_path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    
    let mut writer = BufWriter::new(file);

    for line in &data[..data.len() - 1] {
        write!(writer, "{}", line)?;
    }

    let last_line = &data[data.len() - 1];
    for c in last_line.chars().take(16) {
        if c != '\0' {
            write!(writer, "{}", c)?;
        }
    }
    //writeln!(writer)?;

    Ok(())
}

fn read_matrices_from_file(file_path: &str) -> io::Result<Vec<[[u8; 4]; 4]>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut matrices = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let hex_values: Vec<&str> = line.split_whitespace().collect();
        
        if hex_values.len() != 16 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Número incorreto de valores hexadecimais por linha"));
        }

        let mut matrix = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let index = i * 4 + j;
                matrix[i][j] = u8::from_str_radix(hex_values[index], 16)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            }
        }

        matrices.push(matrix);
    }

    Ok(matrices)
}

fn read_file_as_string(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn write_byte_matrix_to_file(data: &[[[u8; 4]; 4]], file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    for matrix in data {
        for row in matrix.iter() {
            for &b in row.iter() {
                write!(writer, "{:02x} ", b)?;
            }
        }
        writeln!(writer)?;
    }

    Ok(())
}

