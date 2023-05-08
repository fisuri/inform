fn main() {
    let list: Vec<u8> = vec![10, 20, 30];
    let value_str =
        find_element("В векторе 8 элементов, введите нужный вам индекс элемента".to_string());

    match list.get(value_str) {
        Some(el) => println!("{el}"),
        None => println!("Нет такого элемента"),
    }

    print!("{}, {}", value_str, find_avg(&list))
}

fn find_avg(v: &Vec<u8>) -> f64 {
    let mut sum: u8 = 0;
    for i in v {
        sum += i
    }
    let len = (v.len()) as f64;
    let sum = sum as f64;

    sum / len
}

fn find_element(prompt: String) -> usize {
    println!("{}", prompt);

    let mut value_str = String::new();
    std::io::stdin()
        .read_line(&mut value_str)
        .expect("Вы не ввели число");
    let value: usize = match value_str.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Вы не ввели элемент"),
    };
    value
}
