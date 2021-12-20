use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn norm(self) -> f64 {
        (self * self).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, other: Vec3) -> Vec3 {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[derive(Debug)]
pub struct Pendulum {
    pub pos: Vec3,
    pub speed: Vec3,
    pub g: Vec3,
    pub l: f64,
    pub m: f64,
    pub delta_t: f64,
}

impl Pendulum {
    pub fn evolve(&mut self) {
        let new_acc = self.calc_acceleration(self.pos, self.speed);
        self.pos =
            self.pos + self.speed.scale(self.delta_t) + new_acc.scale(self.delta_t.powf(2.0));
        self.speed = self.speed + new_acc.scale(self.delta_t);
    }
    fn calc_t(&self, pos: Vec3, speed: Vec3) -> f64 {
        (pos * self.g + speed * speed * self.m) / self.l
    }

    fn calc_acceleration(&self, pos: Vec3, speed: Vec3) -> Vec3 {
        pos.scale(-self.calc_t(pos, speed) / (self.l * self.m)) + self.g.scale(1.0 / self.m)
    }
}

pub fn pendulum_evolve() -> f64 {
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
    p.pos.norm()
}
