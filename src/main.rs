use lindera::tokenizer::{Token, Tokenizer};
use std::{
    cmp::{max, min},
    env,
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
    let target_text = "仲の直るのもまた早いには早いが。".to_string();
    let tokens = tokenize(&target_text);
    for token in &tokens {
        println!("{:?}", token.detail);
    }
    assert_eq!(tokens.len(), 4);
}

fn find_expression(search_range: usize, tokens: &Vec<Token>) -> Vec<String> {
    let conjections = ["が", "けれど", "けど", "ども", "も", "しかし", "だが"];
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
        let start_index = max(0, i as isize - search_range as isize) as usize;
        let end_index = min(i + search_range, token_length);

        // HACK: もう少しすっきり書けるはず
        let mut has_conjection = false;
        for word_index in i + 1..end_index {
            let word = tokens[word_index].text;
            if conjections.contains(&word) {
                has_conjection = true;
                break;
            }
        }
        if !has_conjection {
            continue;
        }

        for j in start_index..i {
            // 述語が重複しているかを調べる
            let word;
            match tokens[j].detail.get(orth_token_index) {
                Some(v) => word = v,
                None => continue,
            }
            if word == target_word {
                let mut found_expression = String::from("");
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
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
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
