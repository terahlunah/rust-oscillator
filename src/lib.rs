extern crate stopwatch;
use stopwatch::Stopwatch;


pub struct Oscillator {
    stopwatch: Stopwatch,
    frequency: f32,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            stopwatch: Stopwatch::new(),
            frequency: 1.0,
        }
    }

    pub fn start_new() -> Oscillator {
        Oscillator {
            stopwatch: Stopwatch::start_new(),
            frequency: 1.0,
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn get_value(&mut self) -> f32 {
        let time = self.stopwatch.elapsed_ms() as f32 / 1000.0;
        (f32::cos(self.frequency * time * 2.0 * std::f32::consts::PI) + 1.0) / 2.0
    }

    pub fn get_value_scaled(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.get_value()
    }

    pub fn start(&mut self) {
        self.stopwatch.start();
    }

    pub fn stop(&mut self) {
        self.stopwatch.stop();
    }
}
