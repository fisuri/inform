use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let s = "learn learn Rust rust with me me me me me me me me".to_lowercase();

    for i in s.split_whitespace() {
        let cout = map.entry(i).or_insert(0);
        *cout += 1;
    }
    println!("{:?}", map);
}
