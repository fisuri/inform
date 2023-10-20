use std::io;

fn main() {
    loop {
        // Стринговые переменные для записи в них значений пользователя
        let name_str = Pupil::read_volue("Введите ваше имя");
        let age_str = Pupil::read_volue("Введите ваш возраст");
        let wallet_str = Pupil::read_volue("Введите ваше количество денег в кошельке");

        // Конвертация стринговых переменных в другой тип данных
        let name: String = match name_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let age: u8 = match age_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let wallet: f64 = match wallet_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Обращение к структуре Pupil
        let person = Pupil { name, age, wallet };
        let person1 = Pupil {
            name: person.name.clone(),
            ..person
        };

        println!(
            "Имя человека - {}, его возраст - {}, количество денег в его кошельке {}",
            person.name, person.age, person.wallet
        );

        println!(
            "Имя человека - {}, его возраст - {}, количество денег в его кошельке {}",
            person1.name, person1.age, person1.wallet
        );
    }
}

impl Pupil {
    // Записывает значение пользователя
    fn read_volue(prompt: &str) -> String {
        println!("{}", prompt);

        let mut volue: String = String::new();

        io::stdin()
            .read_line(&mut volue)
            .expect("Не удалось прочитать строку");
        volue.trim().to_string()
    }
}

#[derive(Debug)]
struct Pupil {
    name: String,
    age: u8,
    wallet: f64,
}
