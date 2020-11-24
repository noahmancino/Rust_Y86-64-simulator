use std::fs::File;
use std::io::prelude::*;

pub fn tokenize(path: &str) -> Vec<Vec<String>> {
    let mut source = File::open(path).expect("Could not open file at provided path.");
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut line_buffer = String::new();
    source.read_to_string(&mut line_buffer).expect("Could not read file");
    let mut newline_split = line_buffer.split("\n");

    /* Running out of names for different type representations of the same data. This is where I'm
       definitely feeling some hurt from my lack of rust knowledge. */
    for line_slice in newline_split {
        let mut line = String::new();
        println!("{}", line_slice);
        line.push_str(line_slice);
        let line_tokens: Vec<&str> = line.split(" ").collect();
        let mut tokenized_line: Vec<String> = Vec::new();
        for token_slice in line_tokens {
            let mut token = String::new();
            token.push_str(token_slice);
            tokenized_line.push(token);
        }
        lines.push(tokenized_line);
    }
    lines
}