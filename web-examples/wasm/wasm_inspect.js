import fs from "node:fs";

const mode = process.argv[2]
const wasm_path = process.argv[3]

const wasm_buffer = fs.readFileSync(wasm_path);
let module = await WebAssembly.compile(wasm_buffer);

if (mode == "imports") {
  let imports = WebAssembly.Module.imports(module);
  imports.forEach(element => {
    console.log(element);
  });
} else if (mode == "exports") {
  let exports = WebAssembly.Module.exports(module);
  exports.forEach(element => {
    console.log(element);
  });
} else {
  console.log("unknown mode: ", mode);
}