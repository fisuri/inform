pub mod generator;
use generator::gen;

fn main() {
    const DELTA_TIME: f64 = 0.005; //Частота дискретизации сигнала
    const TIME_LIMIT: f64 = 0.5; //Длительность считывания сигнала
    const FREQ: f64 = 7.0; //Частота согласно варианту
    const BIT: u8 = 16; //Разрядность АЦП

    //Получение эталонного сигнала от генератора
    let (ideal_time, ideal_value) = gen(DELTA_TIME, TIME_LIMIT, 0.00001, BIT);
    let (time, analog_to_digital_converter_value) = gen(DELTA_TIME, TIME_LIMIT, FREQ, BIT);

    let analog_to_digital_converter_value_integrated =
        vec![0., analog_to_digital_converter_value.len() as f64];
    let analog_to_digital_converter_value_integrated_sum = analog_to_digital_converter_value[0];

    println!("{:?}", ideal_value);
    println!("{:?}", analog_to_digital_converter_value.len());
}
