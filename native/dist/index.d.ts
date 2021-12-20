declare const c: {
  filtering(v: number[], coefs: number[]): number[];
  filterSum(): number;
  filterSumU64(): number;
  pendulumEvolve(): void;
};
export default c;
