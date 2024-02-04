use core::panic;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_path: &String = &args[1];
    let number_of_files: i32 = match args[2].parse::<i32>() {
        Ok(val) => val,
        Err(_) => panic!("Second argument has to be number of files"),
    };
    let mut rng = rand::thread_rng();
    let mut count = 1;
    while count <= number_of_files {
        let mut file_output_path = PathBuf::from(output_path.clone());
        file_output_path.push(format!("{}.txt", count));
        let random_string = generate_random_string(&mut rng);
        match fs::write(file_output_path, random_string) {
            Ok(_) => println!("Successfully created {}.txt", count),
            Err(err) => panic!("{err}"),
        }
        count += 1;
    }
}

fn generate_random_string(rng: &mut ThreadRng) -> String {
    let limit: u32 = 2621440;
    let start: u32 = 100;
    let mut output = String::new();
    let mut count: u32 = 0;
    let random_range: u32 = rng.gen_range(start..=limit);
    while count < random_range {
        output.push(generate_random_character(rng));
        count += 1;
    }
    output
}

fn generate_random_character(rng: &mut ThreadRng) -> char {
    let random_char: char = if rng.gen_bool(0.5) {
        rng.gen_range('a'..='z')
    } else {
        rng.gen_range('A'..='Z')
    };
    random_char
}
