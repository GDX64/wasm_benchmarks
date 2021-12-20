use node_bindgen::derive::node_bindgen;
mod bench_things;
mod pendulum;
//build with 'nj-cli build'

#[node_bindgen]
pub fn filtering(v: Vec<f64>, coefs: Vec<f64>) -> Vec<f64> {
    bench_things::filtering(v, coefs)
}

#[node_bindgen]
pub fn filter_sum() -> f64 {
    bench_things::filter_sum()
}

#[node_bindgen]
pub fn filter_sum_u64() -> i32 {
    bench_things::filter_sum()
}

#[node_bindgen]
pub fn pendulum_evolve() -> f64 {
    pendulum::pendulum_evolve()
}
