use std::fs::{self, File};
use std::io::Write;

#[warn(dead_code)]
fn read_aozora(path: &str) {
    let file = fs::read(path).unwrap();
    let mut write_file = File::create(format!("output/{}", path)).unwrap();
    let (text, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
    write_file.write(text.into_owned().as_bytes()).unwrap();
}
