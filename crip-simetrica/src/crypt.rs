use crate::cypher::{add_round_key, mix_columns, shift_rows, sub_bytes};

pub fn crypt_blocks(blocks: &[[[u8; 4]; 4]], keys_round: &[[[u8; 4]; 4]; 11]) -> Vec<[[u8; 4]; 4]> {
    let num_blocks = blocks.len();
    let mut blocks_crypt = vec![[[0u8; 4]; 4]; num_blocks];

    for (indice, block) in blocks.iter().enumerate() {
        blocks_crypt[indice] = crypt_block(block, keys_round);
    }

    blocks_crypt
}

fn crypt_block(matrix: &[[u8; 4]; 4], keys_round: &[[[u8; 4]; 4]; 11]) -> [[u8; 4]; 4] {
    let mut matrix_copy = *matrix;

    add_round_key(&mut matrix_copy, keys_round[0]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[1]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[2]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[3]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[4]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[5]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[6]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[7]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[8]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    mix_columns(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[9]);

    sub_bytes(&mut matrix_copy);
    shift_rows(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[10]);

    matrix_copy
}
