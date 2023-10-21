use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_nuber: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Секретное число - {}", secret_nuber);

    loop {
        let mut player_number = String::new();

        println!("Введите число от 1 до 100: ");

        io::stdin()
            .read_line(&mut player_number)
            .expect("Failed to read line");

        let player_number: u8 = match player_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match player_number.cmp(&secret_nuber) {
            Ordering::Less => println!("Бери больше!"),
            Ordering::Equal => {
                println!("Верный ответ");
                break;
            }
            Ordering::Greater => println!("Бери ниже!"),
        }
    }
}
