use lindera::tokenizer::{Token, Tokenizer};
use std::{
    cmp::{max, min},
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

#[test]
fn test_tokenize() {
    let target_text = "今日も晴天です".to_string();
    let tokens = tokenize(&target_text);
    for token in &tokens {
        println!("{:?}", token.detail);
    }
    assert_eq!(tokens.len(), 4);
}

fn find_expression(search_range: isize, tokens: &Vec<Token>) -> Vec<String> {
    let mut expressions = Vec::new();
    let orth_token_index = 6;
    let token_length = tokens.len();
    for (i, value) in tokens.iter().enumerate() {
        let word_category;
        match value.detail.get(1) {
            Some(v) => word_category = v,
            None => continue,
        }
        if word_category != "係助詞" || i == token_length - 1 {
            continue;
        }
        let target_word;
        match tokens[i + 1].detail.get(orth_token_index) {
            Some(v) => target_word = v,
            None => continue,
        };
        let start_index = max(0, i as isize - search_range) as usize;
        for j in start_index..i {
            let word;
            match tokens[j].detail.get(orth_token_index) {
                Some(v) => word = v,
                None => continue,
            }
            if word == target_word {
                let mut found_expression = String::from("");
                let end_index = min(i + 5, token_length);
                for k in j..end_index {
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
    let file = File::open("output/text_data/michikusa.txt").unwrap();
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
