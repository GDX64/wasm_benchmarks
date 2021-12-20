use num::{Num, NumCast};

pub trait GenNum: Num + NumCast + Copy {}

impl GenNum for f64 {}
impl GenNum for u8 {}
impl GenNum for i32 {}

pub fn filtering<N: GenNum>(v: Vec<N>, coefs: Vec<N>) -> Vec<N> {
    (0..v.len() - coefs.len())
        .map(|i| dot(&v[i..], &coefs))
        .collect()
}

pub fn filter_sum<N: GenNum>() -> N {
    let (data, coefs) = data_gen(10_000, 20);
    filtering(data, coefs)
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or(N::zero())
}

fn dot<N: GenNum>(v: &[N], coefs: &[N]) -> N {
    v.iter()
        .zip(coefs)
        .fold(N::zero(), |acc, (&v1, &v2)| v1 * v2 + acc)
}

pub fn data_gen<N: GenNum>(n: usize, n_coefs: usize) -> (Vec<N>, Vec<N>) {
    let data: Vec<N> = (0..n).map(|value| N::from(value).unwrap()).collect();
    let coefs: Vec<N> = (0..n_coefs).map(|value| N::from(value).unwrap()).collect();
    (data, coefs)
}
