# console_log

A `console_log` macro for `web_sys`:

```
console_log!("one");
console_log!("one", "two");
console_log!("one", "two", "three");
console_log!("one", "two", "three", "four");
console_log!("one", "two", "three", "four", "five");
console_log!("one", "two", "three", "four", "five", "six");
console_log!("one", "two", "three", "four", "five", "six", "seven");
```

## testing

Either:

```
$ wasm-pack test --node
```

or

```
$ cargo install wasm-bindgen-cli
$ cargo test --target wasm32-unknown-unknown
```
