use std::io;

fn main() {
    let cat1: f64 = parse_to_f64("Введите первый катет");
    let cat2: f64 = parse_to_f64("Введите второй катет");

    let cat1_tr2: f64 = parse_to_f64("Введите первый катет второго треугольника");
    let cat2_tr2: f64 = parse_to_f64("Введите второй катет второго треугольника");

    let tr1 = Triangle { cat1, cat2 };
    let tr2 = Triangle {
        cat1: cat1_tr2,
        cat2: cat2_tr2,
    };

    let hyp = tr1.find_hyp();
    let area = tr1.find_area();

    let result = tr2.is_eq(area);

    println!(
        "Гипотинуза - {}, площадь - {}, помещается ли tr2 в tr1 - {}",
        hyp, area, result
    )
}

struct Triangle {
    cat1: f64,
    cat2: f64,
}

impl Triangle {
    fn find_hyp(&self) -> f64 {
        (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    }

    fn find_area(&self) -> f64 {
        0.5 * self.cat1 * self.cat2
    }

    fn is_eq(&self, ar: f64) -> bool {
        self.find_area() < ar
    }
}

fn parse_to_f64(prompt: &str) -> f64 {
    println!("{prompt}");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Не удалось прочитать строку");

    value
        .trim()
        .parse::<f64>()
        .expect("Failed to parse input as f64")
}
