use lindera::tokenizer::{Token, Tokenizer};

fn tokenize(target_text: &String) -> Vec<Token> {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer
        .tokenize(&target_text)
        .expect("failed to tokenize");
    return tokens;
}

fn main() {
    let target_text = String::from("とりあえず読むには読んだが、あまりにも難しい。");
    let tokens = tokenize(&target_text);
    for token in tokens {
        println!("{}", token.text);
        println!("{:?}", token.detail);
    }
}
