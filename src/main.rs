use clap::Parser;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = None)]
pub struct InputData {
    /// The output folder where files are generated
    #[arg(index = 1)]
    pub output_path: String,
    /// Number of files to be generated
    #[arg(index = 2)]
    pub number_of_files: u32,
    /// least number of characters in a file (Optional)
    #[arg(short, long, default_value_t = 100)]
    pub lowest_number_of_chars: u32,
    /// highest number of characters in a file (Optional)
    #[arg(short, default_value_t = 2_621_440)]
    pub highest_number_of_chars: u32,
}

fn main() -> Result<(), String> {
    let input_data = InputData::parse();
    let mut rng = rand::thread_rng();
    for count in 1..=input_data.number_of_files {
        let mut file_output_path = PathBuf::from(&input_data.output_path);
        file_output_path.push(format!("{}.txt", count));
        let random_string = generate_random_string(
            &mut rng,
            input_data.lowest_number_of_chars,
            input_data.highest_number_of_chars,
        );
        fs::write(&file_output_path, random_string).map_err(|err| {
            format!(
                "Error writing to file {}: {}",
                file_output_path.display(),
                err
            )
        })?;
    }
    Ok(())
}

fn generate_random_string(
    rng: &mut ThreadRng,
    min_number_of_chars: u32,
    max_number_of_chars: u32,
) -> String {
    let mut output = String::new();
    let mut count: u32 = 0;
    let random_range: u32 = rng.gen_range(min_number_of_chars..=max_number_of_chars);
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
