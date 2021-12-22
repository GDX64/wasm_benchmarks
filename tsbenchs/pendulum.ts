import d from "../native/dist";
import { pendulum_wasm } from "../wasm/pkg/rsw_test";

export function pendulumNative() {
  return d.pendulumEvolve();
}

export function pendulumWasm() {
  return pendulum_wasm();
}

class Vec3 {
  constructor(public x: number, public y: number, public z: number) {}

  dot(other: Vec3) {
    return this.x * other.x + this.y * other.y + this.z * other.z;
  }

  add(other: Vec3) {
    return new Vec3(this.x + other.x, this.y + other.y, this.z + other.z);
  }

  scale(s: number) {
    return new Vec3(this.x * s, this.y * s, this.z * s);
  }

  norm() {
    return Math.sqrt(this.dot(this));
  }
}

class Pendulum {
  g = new Vec3(0, -10, 0);
  l = 1;
  m = 1;
  deltaT = 0.002;

  constructor(public pos: Vec3, public speed: Vec3) {}

  calc_t(pos: Vec3, speed: Vec3): number {
    return (pos.dot(this.g) + this.speed.dot(speed) * this.m) / this.l;
  }

  calc_acceleration(pos: Vec3, speed: Vec3): Vec3 {
    return pos
      .scale(-this.calc_t(pos, speed) / (this.l * this.m))
      .add(this.g.scale(1.0 / this.m));
  }

  evolve() {
    const new_acc = this.calc_acceleration(this.pos, this.speed);
    this.pos = this.pos
      .add(this.speed.scale(this.deltaT))
      .add(new_acc.scale(this.deltaT * this.deltaT));
    this.speed = this.speed.add(new_acc.scale(this.deltaT));
  }
}

export function pendulumJS() {
  const p = new Pendulum(new Vec3(1, 0, 0), new Vec3(0, 0, 0));
  for (let i = 0; i < 10_000; i++) {
    p.evolve();
  }
  return p.pos.norm();
}
