use std::io;

fn main() {
    let arr = (1, 2, 3, 4, 5, 6, 7, 8);
    if let (1, 2, 3, 4, a, 6, 7, b) = arr {
        println!("yes {} {}", a, b);
    } else {
        println!("No")
    }

    let t = Time::Evening("Hello".to_string());
    match &t {
        Time::Morning(s) => println!("{} is morning", s),
        Time::Afternoon(s) => println!("{} is afternoon", s),
        Time::Evening(s) => println!("{} is evening", s),
        Time::Night(s) => println!("{} is night", s),
    }

    if let Time::Morning(m) = t {
        println!("{:?} is morning", m)
    } else {
        println!("is not morning")
    }

    let num: Option<u8> = Some(7);
    match num {
        Some(num) => println!("{:?}", num),
        None => println!("None"),
    }

    if let Some(n) = num {
        println!("{}", n)
    }

    let text = read_user_input("Введите любые числа: ");
    let parsed = text.parse::<i128>();
    let mut num: i128 = 0;

    match parsed {
        Ok(n) => {
            num = n;
            println!("{}", num);
        }
        Err(_) => println!("Введите число"),
    }
    if let Ok(n) = parsed {
        println!("{}", n)
    }

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    if let User { name, age } = user {
        println!("name: {}, age: {}", name, age);
    }

    let list = vec![1, 2, 3, 4, 5, 6];
    for i in 0..10 {
        if let Some(n) = list.get(i) {
            println!("{}", n);
        } else {
            println!("Out of bounds");
        }
    }
}

struct User {
    name: String,
    age: u8,
}

enum Time {
    Morning(String),
    Afternoon(String),
    Evening(String),
    Night(String),
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать ввод пользователя");
    input.trim().to_string()
}
