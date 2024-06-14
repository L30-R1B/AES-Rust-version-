use crate::cypher::S_BOX_MATRIX;

pub const RCON_MATRIX: [[u8; 10]; 4] = [
    [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
];

pub fn round_key_generate(key: [[u8; 4]; 4]) -> [[[u8; 4]; 4]; 11] {
    let mut keys_round = [[[0u8; 4]; 4]; 11];
    keys_round[0] = key;

    keys_round[1] = key_expand(keys_round[0], 0);
    keys_round[2] = key_expand(keys_round[1], 1);
    keys_round[3] = key_expand(keys_round[2], 2);
    keys_round[4] = key_expand(keys_round[3], 3);
    keys_round[5] = key_expand(keys_round[4], 4);
    keys_round[6] = key_expand(keys_round[5], 5);
    keys_round[7] = key_expand(keys_round[6], 6);
    keys_round[8] = key_expand(keys_round[7], 7);
    keys_round[9] = key_expand(keys_round[8], 8);
    keys_round[10] = key_expand(keys_round[9], 9);

    keys_round
}

fn key_expand(round_key: [[u8; 4]; 4], id_rcon_matrix: usize) -> [[u8; 4]; 4] {
    let mut new_round_key = [[0u8; 4]; 4];

    let last_column = [
        S_BOX_MATRIX[(round_key[1][3] >> 4) as usize][(round_key[1][3] & 0x0F) as usize],
        S_BOX_MATRIX[(round_key[2][3] >> 4) as usize][(round_key[2][3] & 0x0F) as usize],
        S_BOX_MATRIX[(round_key[3][3] >> 4) as usize][(round_key[3][3] & 0x0F) as usize],
        S_BOX_MATRIX[(round_key[0][3] >> 4) as usize][(round_key[0][3] & 0x0F) as usize],
    ];

    new_round_key[0][0] = round_key[0][0] ^ last_column[0] ^ RCON_MATRIX[0][id_rcon_matrix];
    new_round_key[1][0] = round_key[1][0] ^ last_column[1] ^ RCON_MATRIX[1][id_rcon_matrix];
    new_round_key[2][0] = round_key[2][0] ^ last_column[2] ^ RCON_MATRIX[2][id_rcon_matrix];
    new_round_key[3][0] = round_key[3][0] ^ last_column[3] ^ RCON_MATRIX[3][id_rcon_matrix];

    new_round_key[0][1] = new_round_key[0][0] ^ round_key[0][1];
    new_round_key[1][1] = new_round_key[1][0] ^ round_key[1][1];
    new_round_key[2][1] = new_round_key[2][0] ^ round_key[2][1];
    new_round_key[3][1] = new_round_key[3][0] ^ round_key[3][1];

    new_round_key[0][2] = new_round_key[0][1] ^ round_key[0][2];
    new_round_key[1][2] = new_round_key[1][1] ^ round_key[1][2];
    new_round_key[2][2] = new_round_key[2][1] ^ round_key[2][2];
    new_round_key[3][2] = new_round_key[3][1] ^ round_key[3][2];

    new_round_key[0][3] = new_round_key[0][2] ^ round_key[0][3];
    new_round_key[1][3] = new_round_key[1][2] ^ round_key[1][3];
    new_round_key[2][3] = new_round_key[2][2] ^ round_key[2][3];
    new_round_key[3][3] = new_round_key[3][2] ^ round_key[3][3];

    new_round_key
}
