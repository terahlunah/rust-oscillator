[![Build Status](https://travis-ci.org/Terah-/rust-oscillator.svg)](https://travis-ci.org/Terah-/rust-oscillator)

# rust-oscillator
An implementation of a simple oscillator in Rust

## Example
```rust
extern crate oscillator;
use oscillator::{Oscillator};

let osc = Oscillator::start_new();
// or
let osc = Oscillator::new();
osc.start();

//...

osc.set_frequency(2.0);
for i in 0..10000 {
    // Use the value
    print!("Oscillator value {}", osc.get_value());
}
```

## Methods
```rust
fn new() -> Oscillator
fn start_new() -> Oscillator
fn start(&mut self)
fn stop(&mut self)
fn get_frequency(&self) -> f32
fn set_frequency(&mut self, freq: f32)
fn get_value(&mut self) -> f32
fn get_value_scaled(&mut self, min: f32, max: f32)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.