import { add, cycle, suite, save, complete } from "benny";
import { jsConv, wasmConv, nativeConv } from "./tsbenchs/convolution";
import { fastJSPendulum } from "./tsbenchs/fastPendulum";
import { pendulumJS, pendulumNative, pendulumWasm } from "./tsbenchs/pendulum";

function benchConv() {
  suite(
    "Convolution",
    add("native conv", nativeConv),
    add("wasm conv", wasmConv),
    add("js conv", jsConv),
    cycle(),
    complete(),
    save({ file: "conv", version: "1.0.0" }),
    save({ file: "conv", format: "chart.html" })
  );
}

function benchPendulum() {
  suite(
    "Pendulum",
    add("wasm", pendulumWasm),
    add("native", pendulumNative),
    add("fast js", fastJSPendulum),
    add("js", pendulumJS),
    cycle(),
    complete(),
    save({ file: "pend", version: "1.0.0" }),
    save({ file: "pend", format: "chart.html" })
  );
  console.log(pendulumNative(), pendulumWasm(), pendulumJS(), fastJSPendulum());
}

benchPendulum();
// benchConv();
