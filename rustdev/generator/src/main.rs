pub mod generator;
use generator::gen;

fn main() {
    const DELTA_TIME: f64 = 0.005; //Частота дискретизации сигнала
    const TIME_LIMIT: f64 = 0.5; //Длительность считывания сигнала
    const FREQ: f64 = 7.0; //Частота согласно варианту
    const BIT: u8 = 16; //Разрядность АЦП

    //Получение эталонного сигнала от генератора
    let (ideal_time, ideal_value) = gen(DELTA_TIME, TIME_LIMIT, 0.00001, BIT);
    let (time, adc_value) = gen(DELTA_TIME, TIME_LIMIT, FREQ, BIT);

    let mut adc_value_integrated = vec![0., adc_value.len() as f64];
    let mut adc_value_integrated_sum = Vec::new();
    adc_value_integrated_sum.push(ideal_value[0]);
    let mut adc_value_different = vec![0., adc_value.len() as f64];

    println!("{:?}", ideal_value);
    println!("{:?}", adc_value.len());

    for i in 1..adc_value.len() {
        adc_value_integrated[i - 1] = ((adc_value[i - 1] + adc_value[i - 1]) / 2.0) * DELTA_TIME;
        adc_value_integrated_sum.push(adc_value_integrated_sum[i - 1] + adc_value_integrated[i]);
        if i != adc_value.len() - 1 {
            adc_value_different[i] = (adc_value[i + 1] - adc_value[i - 1]) / 2.0 * DELTA_TIME;
        }
    }

    println!("Пройденный путь t = 0.1: {:?}", adc_value_integrated[20]);
    println!("Пройденный путь t = 0.5: {:?}", adc_value_integrated[100]);
    println!("Ускорение t = 0.1: {:?}", adc_value_different[20]);
    println!("Ускорение t = 0.5: {:?}", adc_value_different[100]);
}
