use crate::base::{AudioProcessor, Parameter, ParameterId, ParameterValue, MidiReceiver, MidiMessage};
use std::f32::consts::PI;

/// Simple since wave oscillator for test purposes.
#[derive(Debug)]
pub struct Sine {
    /// Amplitude parameter. Valid range is 0.0 to 1.0, default 0.2.
    amplitude: f32,

    /// Frequency parameter in Hz. Valid range is 20 Hz to (SampleRate/2), default 440 Hz.
    frequency: f32,

    /// Number of output channels.
    channel_count: usize,

    /// Phase, used for internal processing.
    phase: f32,

    /// Precalculated phase increment per sample, used for internal processing.
    phase_increment: f32,

    /// Sample rate in Hz.
    sample_rate: f32,
}
#[allow(non_upper_case_globals)]
impl Sine {
    /// Parameter definitions.
    pub const Parameters: [Parameter; 2] = [Sine::Amplitude, Sine::Frequency];
    pub const Amplitude: Parameter = Parameter::new(0, "amplitude");
    pub const Frequency: Parameter = Parameter::new(1, "frequency");

    /// Creates a new sine processor with default parameters.
    pub fn new(channel_count: usize) -> Sine {
        let mut sine = Sine {
            amplitude: 0.2,
            frequency: 440.0,
            channel_count,
            phase: 0.0,
            phase_increment: 0.0,
            sample_rate: 44100.0,
        };
        sine.update_phase_increment();
        sine
    }

    /// Calculates phase increment.
    fn update_phase_increment(&mut self) {
        self.phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
    }
}
#[allow(irrefutable_let_patterns)] // TODO: remove
impl AudioProcessor for Sine {
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        if id == Sine::Amplitude.id { Some(ParameterValue::Float(self.amplitude)) }
        else if id == Sine::Frequency.id { Some(ParameterValue::Float(self.frequency)) }
        else { None }
    }

    fn list_parameters(&self) -> &[Parameter] {
        &Sine::Parameters
    }

    fn process(&mut self, buffer: &mut [f32]) {
        for frame in buffer.chunks_mut(self.channel_count) {
            frame.fill(f32::sin(self.phase) * self.amplitude);
            self.phase += self.phase_increment;
            while self.phase >= 2.0 * PI { self.phase -= 2.0 * PI }
        }
    }

    fn reset(&mut self, sample_rate: f32, _max_buffer_size: usize) {
        self.frequency = self.frequency.clamp(20.0, sample_rate / 2.0); // avoid aliasing
        self.phase = 0.0;
        self.sample_rate = sample_rate;
        self.update_phase_increment();
    }

    fn set_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        if id == Sine::Amplitude.id {
            if let ParameterValue::Float(value) = value {
                self.amplitude = value.clamp(0.0, 1.0);
            }
        }
        else if id == Sine::Frequency.id {
            if let ParameterValue::Float(value) = value {
                self.frequency = value.clamp(20.0, self.sample_rate / 2.0);
                self.update_phase_increment()
            }
        }
    }
}
impl MidiReceiver for Sine {
    fn handle_midi_message(&mut self, message: MidiMessage) {
        match message {
            MidiMessage::NoteOn(_, note, velocity) => {
                // Change frequency and amplitude according to note and velocity.
                self.amplitude = velocity as f32 / 127.0;
                self.frequency = 440.0 * f32::powf(2.0, (note as f32 - 69.0) / 12.0);
                self.update_phase_increment();
            },
            _ => (),
        }
    }
}
