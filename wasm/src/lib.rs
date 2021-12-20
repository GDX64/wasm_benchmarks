use wasm_bindgen::prelude::*;
mod pendulum;

//build with wasm-pack build --target nodejs

#[wasm_bindgen]
pub fn pendulum_wasm() -> f64 {
    pendulum::pendulum_evolve()
}

#[wasm_bindgen]
pub fn filtering(v: Vec<f64>, coefs: Vec<f64>) -> Vec<f64> {
    (0..v.len() - coefs.len())
        .map(|i| dot(&v[i..], &coefs))
        .collect()
}

#[wasm_bindgen]
pub fn filter_sum() -> f64 {
    let (data, coefs) = data_gen();
    filtering(data, coefs).iter().sum()
}

fn dot(v: &[f64], coefs: &[f64]) -> f64 {
    v.iter()
        .zip(coefs)
        .fold(0f64, |acc, (v1, v2)| v1 * v2 + acc)
}

fn data_gen() -> (Vec<f64>, Vec<f64>) {
    let data: Vec<f64> = (0..10_000).map(|value| value as f64).collect();
    let coefs: Vec<f64> = (0..20).map(|value| value as f64).collect();
    (data, coefs)
}
