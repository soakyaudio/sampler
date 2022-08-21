use super::{SamplerVoice, OscillatorSound};
use std::{f32::consts::PI, cell::{RefCell, Ref}, borrow::Borrow, sync::Arc};

/// Oscillator voice for sampler.
pub struct OscillatorVoice {
    /// Sound and MIDI note that is currently playing.
    active_sound: Option<(Arc<OscillatorSound>, u8)>,

    /// Gain applied to sound.
    gain: f32,

    /// Phase, used for internal processing.
    phase: f32,

    /// Precalculated phase increment per sample, used for internal processing.
    phase_increment: f32,

    /// Sample rate in Hz.
    sample_rate: f32,
}
impl OscillatorVoice {
    /// Creates new oscillator voice.
    pub fn new() -> Self {
        OscillatorVoice {
            active_sound: None,
            gain: 0.0,
            phase: 0.0,
            phase_increment: 0.0,
            sample_rate: 44100.0,
        }
    }

    /// Calculates phase increment.
    fn update_phase_increment(&mut self, frequency: f32) {
        self.phase_increment = 2.0 * PI * frequency / self.sample_rate;
    }
}
impl SamplerVoice<OscillatorSound> for OscillatorVoice {
    fn get_active_note(&self) -> Option<u8> {
        if let Some((_, note)) = self.active_sound { Some(note) } else { None }
    }

    fn is_playing(&self) -> bool {
        self.active_sound.is_some()
    }

    fn render(&mut self, buffer: &mut [f32]) {
        if let Some(sound) = &self.active_sound {
            for frame in buffer.chunks_mut(2) { // Sampler expects stereo.
                let sample = sound.0.get_value(self.phase) * self.gain * 0.1; // TODO
                frame.iter_mut().for_each(|s| *s += sample);
                self.phase += self.phase_increment;
                while self.phase >= 2.0 * PI { self.phase -= 2.0 * PI }
            }
        }
    }

    fn reset(&mut self, sample_rate: f32, _max_buffer_size: usize) {
        self.active_sound = None;
        self.sample_rate = sample_rate;
        // Other parameters will be reset on note start.
    }

    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: Arc<OscillatorSound>) {
        let frequency = 440.0 * f32::powf(2.0, (midi_note as f32 - 69.0) / 12.0);
        self.gain = velocity;
        self.active_sound = Some((sound, midi_note));
        self.update_phase_increment(frequency);
    }

    fn stop_note(&mut self, velocity: f32, allow_tail: bool) {
        self.active_sound = None;
    }
}
