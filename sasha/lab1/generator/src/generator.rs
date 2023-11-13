use std::f64::consts::PI;

pub fn gen(time_limit: f64, delta_time: f64) -> (Vec<f64>, Vec<f64>) {
    let n: usize = (time_limit / delta_time).round() as usize;
    let mut time: Vec<f64> = vec![0.; n + 1];
    let mut adc_value: Vec<f64> = vec![0.; n + 1];

    for i in 0..=n {
        if i == 0 {
            time[i] = 0.0;
        } else {
            time[i] = time[i - 1] + delta_time;
        }
        adc_value[i] =
            (7.0 * 2.0 * PI * time[i] as f64).sin() + (16.0 * 2.0 * PI * time[i] as f64).cos();
    }
    (time, adc_value)
}
