use std::env;

mod anchoring;
mod bracket_expresion;
mod constants;
mod file_reader;
mod regex;
mod repetition;
mod repetition_utils;
mod word_searcher;

static INPUT_ARGS_LINE: usize = 3;

fn grep(searcher: word_searcher::Searcher, word: &str, text: &str) {
    let matches_to_print = searcher.search(word, text);
    for match_to_print in &matches_to_print {
        println!("{}", match_to_print);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == INPUT_ARGS_LINE {
        let pattern: &str = &args[1];
        let path_to_file: &str = &args[2];

        match file_reader::FileReader::new(path_to_file) {
            Ok(reader) => {
                let searcher = word_searcher::Searcher::default();
                grep(searcher, pattern, reader.get_text());
            }
            Err(_) => {
                println!("rust_grep: {path_to_file} No such file or directory");
            }
        }
    }
}
