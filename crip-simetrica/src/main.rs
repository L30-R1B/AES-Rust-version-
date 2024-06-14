use std::env;

mod round_key;
mod cypher;
mod blocks_converter;
mod utils;
mod crypt;
mod dcrypt;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Número de argumentos inválidos!");
        return;
    }

    let command = &args[1];
    let file_name = &args[3];
    let key = blocks_converter::block_converter(&args[2]);

    let result:bool = match command.as_str() {
        "CRIPT" => file::text_encoding(file_name, &args[4], &key),
        "DCRIP" => file::text_decoding(file_name, &args[4], &key),
        _ => {
            println!("Comando inválido");
            return;
        }
    };

    if args[5] == "n" && !result {
        file::delete_file_if_exists(file_name);
    }

}