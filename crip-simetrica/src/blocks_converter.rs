use crate::utils::matrix_init;

pub fn blocks_converter(text: &str) -> Vec<[[u8; 4]; 4]> {
    
    let len = text.len();
    let num_blocks = if len % 16 == 0 { len / 16 } else { (len / 16) + 1 };

    let mut blocks = vec![[[0u8; 4]; 4]; num_blocks];
    let mut block_index = 0;
    let mut index = 0;

    while index < len {
        let end = index + 16;
        let sub_text = &text[index..std::cmp::min(end, len)];
        blocks[block_index] = block_converter(sub_text);
        block_index += 1;
        index += 16;
    }

    blocks
}

/*
pub fn blocks_converter(text: &str) -> Vec<[[u8; 4]; 4]> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut blocks = Vec::new();

    for line in lines {
        let len = line.len();
        if len > 16{
            continue;
        }
        let mut index = 0;
        while index < len {
            let end = index + 16;
            let sub_text = &line[index..std::cmp::min(end, len)];
            blocks.push(block_converter(sub_text));
            index += 16;
        }
    }

    blocks
}
*/

fn adjust_string_length(str: &str, length: usize, padding_char: char) -> Vec<u8> {
    let mut sb = str.as_bytes().to_vec();
    while sb.len() < length {
        sb.push(padding_char as u8);
    }
    if sb.len() > length {
        sb.resize(length, padding_char as u8);
    }
    sb
}

pub fn block_converter(block_str: &str) -> [[u8; 4]; 4] {
    let block_str_len_adjust = adjust_string_length(block_str, 16, '\0');
    let array_bytes = block_str_len_adjust.as_slice();
    matrix_init(array_bytes)
}
