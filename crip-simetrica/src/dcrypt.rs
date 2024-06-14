use crate::cypher::{add_round_key, inv_mix_columns, inv_shift_rows, inv_sub_bytes};

pub fn dcrypt_blocks(blocks: &[[[u8; 4]; 4]], keys_round: &[[[u8; 4]; 4]; 11]) -> Vec<[[u8; 4]; 4]> {
    let num_blocks = blocks.len();
    let mut blocks_decrypt = vec![[[0u8; 4]; 4]; num_blocks];

    for (indice, block) in blocks.iter().enumerate() {
        blocks_decrypt[indice] = dcrypt_block(block, keys_round);
    }

    blocks_decrypt
}

fn dcrypt_block(matrix: &[[u8; 4]; 4], keys_round: &[[[u8; 4]; 4]; 11]) -> [[u8; 4]; 4] {
    let mut matrix_copy = *matrix;

    add_round_key(&mut matrix_copy, keys_round[10]);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[9]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[8]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[7]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[6]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[5]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[4]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[3]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[2]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[1]);
    inv_mix_columns(&mut matrix_copy);

    inv_shift_rows(&mut matrix_copy);
    inv_sub_bytes(&mut matrix_copy);
    add_round_key(&mut matrix_copy, keys_round[0]);

    matrix_copy
}
