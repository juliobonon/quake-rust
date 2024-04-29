use std::fs::File;

mod log_reader;
mod parsers;
mod scores;
use log_reader::read_log_file;
use std::env;

fn get_log_file_path() -> String {
    let home = env::var("HOME");
    return format!("{}/qgames.log", home.unwrap());
}

fn main() {
    let file_path = get_log_file_path();
    let file = File::open(file_path).unwrap();

    match read_log_file(file) {
        Ok(result) => {
            println!("{:#?}", result);
            println!("Total games: {}", result.scores.len());
        }
        Err(e) => eprintln!("Error reading log file: {}", e),
    }
}
