fn main() {
    /*let mut counter = Some(0);
    loop {
        match counter {
            Some(value) => {
                if value == 10 {
                    println!("End");
                    counter = None
                } else {
                    println!("{}", value);
                    counter = Some(value + 1);
                }
            }
            None => break,
        }
    }*/
    /*while let Some(value) = counter {
        if value == 10 {
            println!("End");
            counter = None
        } else {
            println!("{}", value);
            counter = Some(value + 1);
        }
    }*/

    let data = vec![
        vec!["Den", "7", "8", "9"],
        vec!["Kate", "4", "5", "6"],
        vec!["John", "1", "2", "3"],
    ];

    for mut student in data {
        println!("{}", student[0]);
        while let Some(value) = student.pop() {
            if let Ok(result) = value.parse::<u8>() {
                println!("{}", result);
            }
        }
    }
}
