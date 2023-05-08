fn main() {
    let mut list = vec![
        Types::Int(4),
        Types::Float(4.4),
        Types::Bool(true),
        Types::Text("hello".to_string()),
    ];

    match &list[0] {
        Types::Int(i) => println!("{}", i),
        Types::Float(f) => println!("{}", f),
        Types::Bool(b) => println!("{}", b),
        Types::Text(t) => println!("{}", t),
    }

    list.push(Types::Int(123));

    println!("{:?}", list.get(2))
}

#[derive(Debug)]
enum Types {
    Int(u8),
    Float(f64),
    Bool(bool),
    Text(String),
}
