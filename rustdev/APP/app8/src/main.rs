use std::{
    fs::{remove_file, rename, File, OpenOptions},
    io::{Read, Write},
};

fn main() {
    println!("Введите путь к файлу: ");

    // Объявляем переменные

    // Записываем путь к файлу
    let path = user_input();

    //Создаем файл с путем и именем из переменной path
    let mut file = File::create(&path).expect("Ошибка в создании файла");

    // Стрнговые переменные для записи в них значений
    let mut file_data = String::new();
    let mut file_data_oo = String::new();

    // Комбайн OpenOptions который позволяет делать все с файлами в одной переменной
    let mut f_oo = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Ошибка в открытии/создании файла");

    println!("Введите текст: ");

    // Берем пользовательский ввод с помощью функции user_input
    let buffer = user_input();

    // Записываем текст из переменной buffer в файл
    file.write_all(&buffer.as_bytes())
        .expect("Ошибка в записи файла");

    // Открываем файл
    let mut f_open = File::open(&path).expect("Ошибка в открытии файла");

    // Читаем данные из файла
    f_open
        .read_to_string(&mut file_data)
        .expect("Ошибка вывода файла");
    println!("1 способ - {:?}", file_data);

    // Читаем данные из файла и передаем в переменную file_data_oo
    f_oo.read_to_string(&mut file_data_oo)
        .expect("Ошибка чтения файла");

    // Записываем текст в файл из переменной file_data_oo
    f_oo.write_all(buffer.as_bytes())
        .expect("Ошибка записи файла");

    println!("2 способ - {:?}", file_data_oo);

    // Переименовываем файл
    rename(path, "temp.txt").expect("Ошибка переименования файла");

    // Удаляем файл
    remove_file("temp.txt").expect("Ошибка удаления файла");
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    // Тримим чтобы не было новой строки от ввода пользователя
    input.trim().to_owned()
}
