use std::f64::consts::PI;

pub fn gen(delta_time: f64, time_limit: f64, freq: f64, bit: u8) -> (Vec<f64>, Vec<f64>) {
    let n = 100;
    let mut t: Vec<f64> = vec![0., n as f64];
    let mut adc_value: Vec<f64> = vec![0., n as f64];

    for i in 0..n {
        if i == 0 {
            t[i] = 0.0;
        } else {
            t[i] = t[i - 1] + delta_time;
            break;
        }
        adc_value[i] =
            (7.0 * 2.0 * PI * t[i]).sin() + (16.0 * 2.0 * PI * t[i]).cos();
    }
    (t, adc_value)
}
