use lindera::tokenizer::{Token, Tokenizer};
use std::cmp::max;

fn tokenize(target_text: &String) -> Vec<Token> {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer
        .tokenize(&target_text)
        .expect("failed to tokenize");
    return tokens;
}

fn find_expression(search_range: usize, tokens: &Vec<Token>) -> Vec<String> {
    let mut expressions = Vec::new();
    let orth_token_index = 6;
    let token_length = tokens.len();
    for (i, value) in tokens.iter().enumerate() {
        if value.text != "は" || i == token_length - 1 {
            continue;
        }
        let target_word = tokens[i + 1].detail.get(orth_token_index).unwrap();
        let start_index = max(0, i - search_range);
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
    let target_text = String::from(
        "とりあえず読むには読んだが、あまりにも難しい。早いは早いがあまり正確ではない。",
    );
    let expressions = find_expression(3, &tokenize(&target_text));
    for expression in expressions {
        println!("{}", expression);
    }
}
