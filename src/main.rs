extern crate rand;

use std::env;
use std::fs;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file_content = read_from_file(filename);

    println!("File content:\n{}", file_content);

    let key = get_key(file_content.len());
    
    let encrypted_bytes = xor_strings(file_content, &key);

    println!("Encrypted string:\n{}", encrypted_bytes);

    write_to_file(&encrypted_bytes);

    let decrypted_bytes = xor_strings(encrypted_bytes, &key);

    println!("Decrypted string:\n{}", decrypted_bytes);
}

fn get_key(length: usize) -> String {
    let mut key_bytes: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        key_bytes.push(rng.gen_range::<u8, u8, u8>(0, 127));
    }

    return convert_bytes_to_str(&key_bytes);
}

fn convert_bytes_to_str(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);

    return s.to_string();
}

fn xor_strings(text: String, key: &String) -> String {
    let mut encoded_bytes: Vec<u8> = vec![];

    for ch in text.chars().zip(key.chars()){
        let (a, b) = ch;
        encoded_bytes.push(a as u8 ^ b as u8);
    }

    return convert_bytes_to_str(&encoded_bytes);
}

fn read_from_file(filename: &String) -> String {
    let file_content = fs::read_to_string(filename).expect("Something went wrong");

    return file_content;
}

fn write_to_file(text: &String) {
    fs::write("encrypted text.txt", text).expect("Something went wrong while writing to file");
}