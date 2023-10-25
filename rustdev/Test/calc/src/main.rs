use std::io;

fn main() {
    let x = input("Введите первое число: ".to_string());
    let y = input("Введите второе число: ".to_string());

    let x: f64 = x
        .parse()
        .expect("Не удалось запарсить значение, напишите цифры");
    let y: f64 = y
        .parse()
        .expect("Не удалось запарсить значение, напишите цифры");

    let r = x + y;

    println!("{} + {} = {}", x, y, r);
}

fn input(prompt: String) -> String {
    println!("{prompt}");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Не удалось считать строку");
    s.trim().to_string()
}
