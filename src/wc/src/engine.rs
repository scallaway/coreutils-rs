use crate::flags::Flags;
use core::str::FromStr;
use std::fs;

// TODO: Handle reading from stdin
#[derive(Debug)]
pub struct Engine {
    pub file_name: String,
    pub file: String,
    // This is optional as there may not be any flags, in the event that we're
    // just displaying the default values
    pub flag: Option<Flags>,
}

impl Engine {
    pub fn new(args: Vec<String>) -> Engine {
        Engine {
            file: Engine::read_file_from_arg(&args),
            // TODO: In an idea world we wouldn't convert this to a string, and
            //       just keep the references intact.
            file_name: Engine::get_file_name(&args).to_string(),
            flag: if &args.len() > &1 {
                Some(
                    Flags::from_str(&args.get(0).unwrap())
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
            Flags::Bytes => self.file.as_bytes().len(),
            Flags::Lines => self.file.lines().count(),
            Flags::Chars => self.file.chars().collect::<Vec<char>>().len(),
            Flags::Words => self.get_word_count(),
        };

        println!("{}", format!("{} {}", statistic, self.file_name));
    }

    fn run_default(&self) {
        let lines = self.file.lines().count();
        let words = self.get_word_count();
        let bytes = self.file.as_bytes().len();

        println!(
            "{}",
            format!("{} {} {} {}", lines, words, bytes, self.file_name)
        );
    }

    // We read the file into a string, as we can simply convert to bytes
    // when required, and that's faster than storing the file in two different
    // formats in memory
    fn read_file_from_arg(args: &Vec<String>) -> String {
        // TODO: Actually handle this error
        fs::read_to_string(Engine::get_file_name(args))
            .expect("Unable to find file")
    }

    fn get_file_name(args: &Vec<String>) -> &String {
        if args.get(1).is_none() {
            args.get(0).unwrap()
        } else {
            args.get(1).unwrap()
        }
    }

    fn get_word_count(&self) -> usize {
        self.file.lines().fold(0, |acc, line| {
            acc + line.split_whitespace().collect::<Vec<&str>>().len()
        })
    }
}

// TODO: Write tests for this
