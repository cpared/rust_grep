use std::env;

mod file_reader;
mod word_searcher;
mod regex;
mod bracket_expresion;

static INPUT_ARGS_LINE: usize = 3;

fn grep(searcher:word_searcher::Searcher, word: &String, text: &String) {
    let matches_to_print = searcher.search(word, text);
    for match_to_print in &matches_to_print {
        println!("{}", match_to_print);
    }
}

// grep  searches  for  PATTERNS in each FILE.  PATTERNS is one or more patterns separated by newline characters,
// and grep prints each line that matches a pattern.  Typically PATTERNS should be quoted when grep is used in  a
// shell command.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == INPUT_ARGS_LINE {
        let pattern: &String = &args[1];
        let path_to_file: &String = &args[2];

        match file_reader::FileReader::new(path_to_file) {
            Ok(reader) => {
                let searcher = word_searcher::Searcher::new();
                grep(searcher, pattern, reader.get_text());
            }
            Err(_) => {
                println!("rust_grep: {path_to_file} No such file or directory");
            }
        }
    }
}