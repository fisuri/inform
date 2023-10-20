fn main() {
    let math = Math::Add(5.0, 4.0);

    let result = math.mo();

    println!("{result}")
}
enum Math {
    Add(f64, f64),
    Min(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

impl Math {
    fn mo(&self) -> f64 {
        match self {
            Math::Add(a, b) => a + b,
            Math::Min(a, b) => a - b,
            Math::Mul(a, b) => a * b,
            Math::Div(a, b) => a / b,
        }
    }
}
