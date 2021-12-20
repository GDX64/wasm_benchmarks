use rand::random;
use std::time::Instant;

mod bench_things;
mod pendulum;

fn main() {
    my_bench(bench_filtering(), "filter");
    my_bench(
        || {
            bench_things::filter_sum::<f64>();
        },
        "filter sum",
    );
}

fn bench_filtering() -> impl FnOnce() {
    let data: Vec<f64> = (0..10_000).map(|_| random::<f64>()).collect();
    let coefs: Vec<f64> = (0..20).map(|_| random::<f64>()).collect();
    move || {
        bench_things::filtering(data, coefs);
    }
}

fn my_bench(func: impl FnOnce() -> (), name: &str) {
    let now = Instant::now();
    func();
    println!("{}: {:?}", name, 1e9 / (now.elapsed().as_nanos() as f64));
}

#[cfg(test)]
mod tests {
    use crate::bench_things::data_gen;

    #[test]
    fn generic_test() {
        assert_eq!((vec![0], vec![0]), data_gen::<u8>(1, 1))
    }
}

#[cfg(test)]
mod test {
    use num::Float;

    use super::pendulum::{Pendulum, Vec3};

    #[test]
    fn add_vec() {
        let result = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }
    #[test]
    fn mult_vec() {
        let result = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(result, 14.0);
    }

    #[test]
    fn pendulum() {
        let mut p = Pendulum {
            delta_t: 0.002,
            pos: Vec3::new(1.0, 0.0, 0.0),
            speed: Vec3::new(0.0, 0.0, 0.0),
            g: Vec3::new(0.0, -10.0, 0.0),
            l: 1.0,
            m: 1.0,
        };
        (0..10_000).for_each(|_| {
            p.evolve();
        });
        println!("{}", p.pos.norm())
    }
}
