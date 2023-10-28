pub mod generator;
use generator::gen;

fn main() {
    const DELTA_TIME: f64 = 0.005; //Частота дискретизации сигнала
    const TIME_LIMIT: f64 = 0.6; //Длительность считывания сигнала

    //Получение эталонного сигнала от генератора
    let (ideal_time, ideal_value) = gen(TIME_LIMIT, 0.00001);
    let (time, adc_value) = gen(TIME_LIMIT, DELTA_TIME);

    let mut adc_value_different: Vec<f64> = vec![0.; adc_value.len()];
    let mut adc_value_integrated: Vec<f64> = vec![0.; adc_value.len()];
    let mut adc_value_integrated_sum: Vec<f64> = Vec::new();
    adc_value_integrated_sum.push(ideal_value[0]);

    println!("{:?}", ideal_value);
    println!("{:?}", adc_value.len());

    for i in 1..adc_value.len() {
        adc_value_integrated[i - 1] = ((adc_value[i] + adc_value[i - 1]) / 2.0) * DELTA_TIME;
        adc_value_integrated_sum.push(adc_value_integrated_sum[i - 1] + adc_value_integrated[i]);
        if i != adc_value.len() - 1 {
            adc_value_different[i] = (adc_value[i + 1] - adc_value[i - 1]) / 2.0 * DELTA_TIME;
        }
    }

    println!(
        "Пройденный путь t = 0.1: {:?}",
        adc_value_integrated[(0.1 / DELTA_TIME).round() as usize]
    );
    println!(
        "Пройденный путь t = 0.5: {:?}",
        adc_value_integrated[(0.5 / DELTA_TIME).round() as usize]
    );
    println!(
        "Ускорение t = 0.1: {:?}",
        adc_value_different[(0.1 / DELTA_TIME).round() as usize]
    );
    println!(
        "Ускорение t = 0.5: {:?}",
        adc_value_different[(0.5 / DELTA_TIME).round() as usize]
    );
}
