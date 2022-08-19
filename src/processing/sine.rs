use crate::base::{AudioProcessor, ParameterId, ParameterValue};
use std::f32::consts::PI;

/// Simple since wave oscillator for test purposes.
#[derive(Debug)]
pub struct Sine {
    /// Amplitude parameter. Valid range is 0.0 to 1.0, default 0.2.
    amplitude: (f32, ParameterId),

    /// Frequency parameter in Hz. Valid range is 20 Hz to (SampleRate/2), default 440 Hz.
    frequency: (f32, ParameterId),

    /// Number of output channels.
    channel_count: usize,

    /// Phase, used for internal processing.
    phase: f32,

    /// Precalculated phase increment per sample, used for internal processing.
    phase_increment: f32,

    /// Sample rate in Hz.
    sample_rate: f32,
}
impl Sine {
    /// Creates a new [Sine] with default parameters.
    pub fn new(channel_count: usize, sample_rate: f32) -> Sine {
        let mut sine = Sine {
            amplitude: (0.2, ParameterId::new("amplitude")),
            frequency: (440.0, ParameterId::new("frequency")),
            channel_count,
            phase: 0.0,
            phase_increment: 0.0,
            sample_rate,
        };
        sine.update_phase_increment();
        sine
    }

    /// Calculates phase increment.
    fn update_phase_increment(&mut self) {
        self.phase_increment = 2.0 * PI * self.frequency.0 / self.sample_rate;
    }
}
#[allow(irrefutable_let_patterns)] // TODO: remove
impl AudioProcessor for Sine {
    fn get_parameter(&self, id: &ParameterId) -> Option<ParameterValue> {
        if *id == self.amplitude.1 { Some(ParameterValue::Float(self.amplitude.0)) }
        else if *id == self.frequency.1 { Some(ParameterValue::Float(self.frequency.0)) }
        else { None }
    }

    fn process(&mut self, buffer: &mut [f32]) {
        for frame in buffer.chunks_mut(self.channel_count) {
            frame.fill(f32::sin(self.phase) * self.amplitude.0);
            self.phase += self.phase_increment;
        }
    }

    fn reset(&mut self, sample_rate: f32, _max_buffer_size: usize) {
        self.frequency.0 = self.frequency.0.clamp(20.0, sample_rate / 2.0); // avoid aliasing
        self.phase = 0.0;
        self.sample_rate = sample_rate;
        self.update_phase_increment();
    }

    fn set_parameter(&mut self, id: &ParameterId, value: &ParameterValue) {
        if *id == self.amplitude.1 {
            if let ParameterValue::Float(value) = value {
                self.amplitude.0 = value.clamp(0.0, 1.0);
            }
        }
        else if *id == self.frequency.1 {
            if let ParameterValue::Float(value) = value {
                self.frequency.0 = value.clamp(20.0, self.sample_rate / 2.0);
                self.update_phase_increment()
            }
        }
    }
}
