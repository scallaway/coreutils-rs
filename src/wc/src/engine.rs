use crate::flags::Flags;
use core::str::FromStr;
use std::{env::Args, fs};

// TODO: Handle reading from stdin
#[derive(Debug)]
pub struct Engine {
    pub file_name: String,
    // This is optional as there may not be any flags, in the event that we're
    // just displaying the default values
    pub flag: Option<Flags>,
}

/// The beating heart of the program, where all the computation takes place
impl Engine {
    pub fn new(args: Args) -> Engine {
        // We can always start from the second argument in the list as the
        // first is _always_ the program name
        let args = args.collect::<Vec<String>>()[1..].to_vec();

        let file_name = get_file_name_from_args(&args);

        Engine {
            file_name,
            flag: if &args.len() > &1 {
                Some(
                    Flags::from_str(&get_flag_from_args(&args))
                        // TODO: Don't panic here
                        .expect("Could not parse flag"),
                )
            } else {
                None
            },
        }
    }

    pub fn run(&self) {
        if self.flag.is_none() {
            self.run_default();
            return;
        }

        let statistic = match self.flag.unwrap() {
            Flags::Bytes => fs::metadata(&self.file_name)
                .expect("Couldn't find file specified")
                .len() as usize,
            Flags::Lines => self.read_file(&self.file_name).lines().count(),
            Flags::Chars => self
                .read_file(&self.file_name)
                .chars()
                .collect::<Vec<char>>()
                .len(),
            Flags::Words => {
                self.get_word_count(&self.read_file(&self.file_name))
            },
            Flags::LongestLine => {
                self.get_longest_line(&self.read_file(&self.file_name))
            },
        };

        println!("{}", format!("{} {}", statistic, self.file_name));
    }

    fn run_default(&self) {
        let file = self.read_file(&self.file_name);
        let lines = file.lines().count();
        let words = self.get_word_count(&file);
        let bytes = file.as_bytes().len();

        println!(
            "{}",
            format!("{} {} {} {}", lines, words, bytes, self.file_name)
        );
    }

    // Read the file into a String here as it makes it easier to work with
    fn read_file(&self, file_name: &String) -> String {
        // TODO: Don't panic
        fs::read_to_string(file_name).expect("Unable to find file")
    }

    fn get_word_count(&self, file: &String) -> usize {
        file.lines().fold(0, |acc, line| {
            acc + line.split_whitespace().collect::<Vec<&str>>().len()
        })
    }

    fn get_longest_line(&self, file: &String) -> usize {
        file.lines().fold(
            0,
            |acc, line| {
                if line.len() > acc {
                    line.len()
                } else {
                    acc
                }
            },
        )
    }
}

// TODO: Probably move this elsewhere
/// Returns the first string that it finds that doesn't start with a hyphen
fn get_file_name_from_args(args: &Vec<String>) -> String {
    return args
        .iter()
        .find(|&arg| !arg.starts_with("-"))
        .expect("File name was not supplied with invocation of program")
        .to_owned();
}

/// Returns the first string that it find that does start with a hyphen
fn get_flag_from_args(args: &Vec<String>) -> String {
    return args
        .iter()
        .find(|&arg| arg.starts_with("-"))
        .unwrap()
        .to_owned();
}

// TODO: Write tests for this
