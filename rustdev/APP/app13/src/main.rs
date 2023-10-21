use std::io;

fn main() {
    let is_even = |a: i32| a % 2 == 0;

    let mut input = |prompt: String| -> i32 {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        input.trim().parse().expect("Ошибка парсинга числа")
    };
    println!("{}", is_even(input("Введите число: ".to_string())));

    let add = |a: i32, b: i32| a + b;
    println!(
        "{}",
        add(
            input("Введите первое число : ".to_string()),
            input("Введите второе число: ".to_string())
        )
    );

    let mul = |a: i32| a + input("Введите число: ".to_string());
    println!("{}", mul(5));14
}

/*fn is_even(n: i32) -> bool {
    n % 2 == 0
}*/
