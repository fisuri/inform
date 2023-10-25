use std::f64::consts::PI;

pub fn gen(delta_time: f64, time_limit: f64, freq: f64, bit: u8) -> (Vec<f64>, Vec<f64>) {
    let n = time_limit / delta_time;
    let mut t: Vec<f64> = vec![0., n];
    let mut analog_to_digital_converter_value: Vec<f64> = vec![0., n];

    for i in 0..n {
        if i == 0 {
            t[i] = 0.0;
        } else {
            t[i] = t[i - 1] + delta_time;
        }
        analog_to_digital_converter_value[i] =
            (7.0 * 2.0 * PI * t[i]).sin() + (16.0 * 2.0 * PI * t[i]).cos();
    }
    (t, analog_to_digital_converter_value)
}
