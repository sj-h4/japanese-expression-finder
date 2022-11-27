pub mod read_aozora;
use lindera::tokenizer::{Token, Tokenizer};
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

fn tokenize(target_text: &String) -> Vec<Token> {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer
        .tokenize(&target_text)
        .expect("failed to tokenize");
    return tokens;
}

fn find_expression(search_range: isize, tokens: &Vec<Token>) -> Vec<String> {
    let mut expressions = Vec::new();
    let orth_token_index = 6;
    let token_length = tokens.len();
    for (i, value) in tokens.iter().enumerate() {
        if value.text != "は" || i == token_length - 1 {
            continue;
        }
        let target_word = tokens[i + 1].detail.get(orth_token_index).unwrap();
        let start_index = max(0, i as isize - search_range) as usize;
        for j in start_index..i {
            let word = tokens[j].detail.get(orth_token_index).unwrap();
            if word == target_word {
                let mut found_expression = String::from("");
                for k in j..i + 2 {
                    found_expression += tokens[k].text;
                }
                expressions.push(found_expression);
                continue;
            }
        }
    }
    return expressions;
}

fn main() {
    let file = File::open("output/aozora.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = tokenize(&line);
        let expressions = find_expression(3, &tokens);
        for expression in expressions {
            println!("{}", expression);
        }
    }
}
