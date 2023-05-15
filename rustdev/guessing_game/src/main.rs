use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let input = |prompt: &str| -> u8 {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Неверное значение!");
                0
            }
        }
    };

    let secret_number: u8 = rand::thread_rng().gen_range(0..=100);

    loop {
        let input_user = input("Введите любое число от 0 до 100");

        println!("Ваше число {}", input_user);

        match input_user.cmp(&secret_number) {
            Ordering::Less => println!("Больше число!"),
            Ordering::Equal => {
                println!("Вы попали");
                break;
            }
            Ordering::Greater => println!("Число ниже!"),
        };
    }
}
