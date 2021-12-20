import { filtering, filter_sum } from "../wasm/pkg/rsw_test";
import d from "../native/dist";
import { range } from "ramda";

console.log(d.filterSum);

const N = 10_000;
const N_COEFS = 20;

function randArray(n: number, ceil: number) {
  return range(0, n).map(() => Math.random() * ceil);
}

export function wasmConv() {
  return filter_sum();
}

export function nativeConv() {
  return d.filterSum();
}

export function jsConv() {
  const { arrData, coefs } = genData();
  return arrSum(jsImperativeFiltering(arrData, coefs));
}

function rangeArr(n: number) {
  const arr = new Float64Array(n);
  for (let i = 0; i < N; i++) {
    arr[i] = i;
  }
  return arr;
}

function genData() {
  const arrData = rangeArr(N);
  const coefs = rangeArr(N_COEFS);
  return { arrData, coefs };
}

function jsImperativeFiltering(v: Float64Array, coefs: Float64Array) {
  const result = new Float64Array(v.length - coefs.length);
  for (let i = 0; i < v.length - coefs.length + 1; i++) {
    let filtered = 0;
    for (let j = 0; j < coefs.length; j++) {
      filtered += coefs[j] * v[i + j];
    }
    result[i] = filtered;
  }
  return result;
}

function arrSum(arr: Float64Array) {
  let sum = 0;
  const n = arr.length;
  for (let i = 0; i < n; i++) {
    sum += arr[i];
  }
  return sum;
}
