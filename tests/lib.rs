#![allow(dead_code)]

extern crate oscillator;
extern crate stopwatch;

use oscillator::Oscillator;
use stopwatch::Stopwatch;

fn assert_near(a : f32, b : f32) {
    assert!((a-b).abs() < 0.05)
}

#[test]
fn test_new() {
    let mut osc = Oscillator::new();
    assert_eq!(osc.get_value(), 1.0);
}

#[test]
fn test_null_freq() {
    let mut osc = Oscillator::start_new();
    osc.set_frequency(0.0);
    assert_eq!(osc.get_value(), 1.0);
}

#[test]
fn test_cycle() {
    let mut timer = Stopwatch::start_new();
    let mut osc = Oscillator::start_new();

    assert_near(osc.get_value(), 1.0);

    while timer.elapsed_ms() < 500 { }

    assert_near(osc.get_value(), 0.0);

    while timer.elapsed_ms() < 1000 { }

    assert_near(osc.get_value(), 1.0);
}

#[test]
fn test_freq() {
    let mut timer = Stopwatch::start_new();
    let mut osc = Oscillator::start_new();
    osc.set_frequency(0.5);

    assert_near(osc.get_value(), 1.0);

    while timer.elapsed_ms() < 1000 { }

    assert_near(osc.get_value(), 0.0);

    while timer.elapsed_ms() < 2000 { }

    assert_near(osc.get_value(), 1.0);
}
