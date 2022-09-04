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
    #[allow(dead_code)]
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

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_note() {
        let mut buffer: Box<[f32]> = vec![0.0; 512].into_boxed_slice();
        let mut voice = OscillatorVoice::new();
        let sound = OscillatorSound::new();
        voice.reset(1000.0, 512);

        voice.start_note(48, 1.0, Arc::new(sound), 0);
        assert_eq!(voice.get_active_note(), Some(48));

        voice.stop_note(0.0, true);
        assert_eq!(voice.is_playing(), true); // allow tail

        voice.render(&mut buffer);
        assert_eq!(voice.is_playing(), false);
    }

    #[test]
    fn render_sound() {
        let mut buffer: Box<[f32]> = vec![0.0; 1024].into_boxed_slice();
        let mut voice = OscillatorVoice::new();
        let sound = OscillatorSound::new();
        voice.reset(1000.0, buffer.len());

        voice.start_note(69, 1.0, Arc::new(sound), 0); // note 69 = A4 = 440Hz
        voice.render(&mut buffer);

        for i in (128..buffer.len()).step_by(2) { // skip adsr attack, stereo voice
            let value = f32::sin(2.0 * PI * 440.0 * (i/2) as f32 / 1000.0) * 0.1; // 2 * PI * frequency * i / sample_rate
            assert!((buffer[i] - value).abs() < 1e-4, "Unexpected buffer value at index {}: got {} instead of {}", i, buffer[i], value);
            assert_eq!(buffer[i], buffer[i+1]);
        }
    }
}
