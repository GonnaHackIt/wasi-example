# WASI Example

Example using WASI component model. Showcasing scenario where Wasm component written in Rust (possibly some server) uses Wasm component written in JavaScript
to validate data.

# Running

To run a program:

1. Add a wasmtime runtime to the PATH
2. Run in console: wasmtime run command.wasm SOME_EMAIL SOME_PHONE_NUMBER