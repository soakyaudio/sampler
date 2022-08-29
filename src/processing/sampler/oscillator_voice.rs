use super::{SamplerVoice, OscillatorSound, LinearAdsr};
use std::{f32::consts::PI, sync::Arc};

/// Oscillator voice for sampler.
pub struct OscillatorVoice {
    /// Sound and MIDI note that is currently playing.
    active_sound: Option<(Arc<OscillatorSound>, u8)>,

    /// ADSR envelope.
    adsr: LinearAdsr,

    /// Gain applied to sound.
    gain: f32,

    /// Key down state.
    key_down: bool,

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
            adsr: LinearAdsr::new(0.03, 0.1),
            gain: 0.0,
            key_down: false,
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

    fn get_priority(&self) -> u32 {
        0
    }

    fn is_key_down(&self) -> bool {
        self.key_down
    }

    fn is_playing(&self) -> bool {
        self.active_sound.is_some()
    }

    fn render(&mut self, buffer: &mut [f32]) {
        if let Some(sound) = &self.active_sound {
            for frame in buffer.chunks_mut(2) { // Sampler expects stereo.
                let envelope_gain = self.adsr.next_sample();
                let sample = sound.0.get_value(self.phase) * self.gain * envelope_gain * 0.1; // TODO
                frame.iter_mut().for_each(|s| *s += sample);
                self.phase += self.phase_increment;
                while self.phase >= 2.0 * PI { self.phase -= 2.0 * PI }

                // Stop note after envelope finished release stage.
                if !self.adsr.is_active() { self.stop_note(0.0, false); break; }
            }
        }
    }

    fn reset(&mut self, sample_rate: f32, _max_buffer_size: usize) {
        self.active_sound = None;
        self.adsr.reset(sample_rate);
        self.sample_rate = sample_rate;
        // Other parameters will be reset on note start.
    }

    fn set_key_down(&mut self, key_down: bool) {
        self.key_down = key_down;
    }

    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: Arc<OscillatorSound>, _initial_priority: u32) {
        let frequency = 440.0 * f32::powf(2.0, (midi_note as f32 - 69.0) / 12.0);
        self.active_sound = Some((sound, midi_note));
        self.adsr.note_on();
        self.gain = velocity;
        self.phase = 0.0;
        self.update_phase_increment(frequency);
    }

    fn stop_note(&mut self, _velocity: f32, allow_tail: bool) {
        if allow_tail {
            self.adsr.note_off();
        } else {
            self.active_sound = None;
        }
    }
}
