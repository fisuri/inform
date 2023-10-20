use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    let mut path = String::new();
    let mut path2 = String::new();
    let mut path3 = String::new();

    println!("Введите путь к файлу: ");
    std::io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    // Обработка ошибки открытия файла через match
    let f = match File::open(&path) {
        Ok(o) => o,
        Err(e) => match e.kind() {
            // Обработка ошибки типа NotFound
            ErrorKind::NotFound => match File::create(&path) {
                Ok(o) => o,
                Err(e) => panic!("Ошибка в создании файла: {}", e),
            },
            other => panic!("Ошибка: {}", other),
        },
    };

    println!("Введите путь ко второму файлу: ");
    std::io::stdin()
        .read_line(&mut path2)
        .expect("Failed to read line");

    /* Обработка ошибки открытия файла через .unwrap()
    Ok(file) => file,
    Err(e) => panic!("...{:?}, e") */

    let f1 = File::open(&path2).unwrap();

    // Обработка ошибки открытия файла через .expect()
    let f2 = File::open(&path2).expect("Не удалось открыть файл");

    println!("Введите путь к третьему файлу: ");
    std::io::stdin()
        .read_line(&mut path3)
        .expect("Failed to read line");

    // Обработка ошибки чтения файла через match
    match read_file(&path3) {
        Ok(data) => println!("Данные файла: {}", data),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(&path3) {
                Ok(_) => println!("Файл успешно создан"),
                Err(e) => panic!("Ошибка в создании файла: {}", e),
            },
            other => panic!("Ошибка: {:?}", other),
        },
    }
}

// Функция чтения файла
// Обработка ошибки чтения файла через оператор "?"
fn read_file(path: &String) -> Result<String, Error> {
    let mut data = String::new();
    File::open(path)?.read_to_string(&mut data)?;
    Ok(data)
}
