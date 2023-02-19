# jsonmpk

Implemented mutual conversion between json value and mpk

## ToRmp

```rust
use jsonmpk::ToRmp;
// ...
let foo: T;
let value: Value = serde_json::to_value(foo).unwrap();
let rmp: Vec<u8> = value.to_rmp().unwrap();
```

## FromRmp

```rust
use jsonmpk::FromRmp;
// ...
let rmp: Vec<u8>;
let value: Value = Value::from_rmp(&rmp).unwrap()
let foo: T = serde_json::from_value(value).unwrap();
```
