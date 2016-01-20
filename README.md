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
