use std::io;

fn main() {
    let x = input();
    let y = input();

    let x: i8 = x
        .parse()
        .expect("Не удалось запарсить значение, напишите цифры");
    let y: i8 = y
        .parse()
        .expect("Не удалось запарсить значение, напишите цифры");

    let r = x + y;

    println!("{} + {} = {}", x, y, r);
}

fn input() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Не удалось считать строку");
    s.trim().to_string()
}
