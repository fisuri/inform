fn main() {
    let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut vec_capacity: Vec<u8> = Vec::with_capacity(5);

    vec.push(1);
    vec.insert(0, 0);

    println!("Vec - {:?}", &vec);

    match vec.first() {
        Some(x) => println!("Первый элемент - {:?}", x),
        None => println!("Нет элемента"),
    }

    match vec.last() {
        Some(x) => println!("Последний элемент - {:?}", x),
        None => println!("Нет элемента"),
    }

    if vec.is_empty() {
        println!("Вектор пустой");
    } else {
        println!(
            "Первый элемент - {}, Последний элемент - {}",
            vec[0],
            vec[vec.len() - 1]
        );
    }

    match vec.get(2) {
        Some(x) => println!("Получение второго индекса из вектора - {:?}", x),
        None => println!("Индекс не найден"),
    }

    vec_capacity.push(1);
    vec_capacity.push(2);
    vec_capacity.push(3);
    vec_capacity.push(4);
    vec_capacity.push(5);
    println!(
        "Емкость Vec_capacity - {}, vec_capacity - {:?}",
        vec_capacity.capacity(),
        vec_capacity
    );

    println!("Удаление индекса из вектора - {}", vec.remove(2));

    let rev = match vec.pop() {
        Some(x) => x,
        None => 0,
    };
    println!("Удаленный элемент массива {:?}", rev);

    println!("Емкость вектора - {}, Vec - {:?}", vec.capacity(), vec);

    println!("Вектор - {:?}", vec.truncate(8));

    vec.append(&mut vec_capacity);
    println!(
        "Первый вектор - {:?}, Второй вектор - {:?}",
        vec, vec_capacity
    );

    vec.clear();
    println!(
        "Очистка вектора\nЕмкость вектора - {}, Vec - {:?}",
        vec.capacity(),
        vec
    );
}
