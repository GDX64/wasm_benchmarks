type Vec3 = [number, number, number];

function dot(v: Vec3, other: Vec3) {
  return v[0] * other[0] + v[1] * other[1] + v[2] * other[2];
}

function add(v: Vec3, other: Vec3): Vec3 {
  return [v[0] + other[0], v[1] + other[1], v[2] + other[2]];
}

function scale(v: Vec3, s: number): Vec3 {
  return [v[0] * s, v[1] * s, v[2] * s];
}

function norm(v: Vec3) {
  return Math.sqrt(dot(v, v));
}

class Pendulum {
  g = [0, -10, 0] as Vec3;
  l = 1;
  m = 1;
  deltaT = 0.002;

  constructor(public pos: Vec3, public speed: Vec3) {}

  calcT(pos: Vec3, speed: Vec3): number {
    return (dot(pos, this.g) + dot(speed, speed) * this.m) / this.l;
  }

  calcAcceleration(pos: Vec3, speed: Vec3): Vec3 {
    const result = scale(pos, -this.calcT(pos, speed) / (this.l * this.m));
    const g = scale(this.g, 1.0 / this.m);
    return [result[0] + g[0], result[1] + g[1], result[2] + g[2]];
  }

  evolve() {
    const acc = this.calcAcceleration(this.pos, this.speed);
    this.pos = add(
      add(this.pos, scale(this.speed, this.deltaT)),
      scale(acc, this.deltaT * this.deltaT)
    );
    this.speed = add(this.speed, scale(acc, this.deltaT));
  }
}

export function fastJSPendulum() {
  const p = new Pendulum([1, 0, 0], [0, 0, 0]);
  for (let i = 0; i < 10_000; i++) {
    p.evolve();
  }
  return norm(p.pos);
}
