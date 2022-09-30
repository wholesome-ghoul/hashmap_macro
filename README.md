# HashMap Macro

Creates a HashMap from provided key/value pairs.

## Usage

```rust
use std::collections::HashMap;
use hashmap_macro::hashmap;

let m: HashMap<&str, u32> = hashmap![];
let m: HashMap<&str, u32> = hashmap!("a" => 1);
let m: HashMap<&str, u32> = hashmap! {
    "a" => 1,
    "b" => 2,
};
```

Inspired by Jon Gjengset's [Crust of Rust: Declarative Macros](https://youtu.be/q6paRBbLgNw)
