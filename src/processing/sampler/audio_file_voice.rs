use super::{SamplerVoice, AudioFileSound, LinearAdsr};
use std::sync::Arc;

/// Audio file voice for sampler.
pub struct AudioFileVoice {
    /// Sound and MIDI note that is currently playing.
    active_sound: Option<(Arc<AudioFileSound>, u8)>,

    /// ADSR envelope.
    adsr: LinearAdsr,

    /// Gain applied to sound.
    gain: f32,

    /// Key down state.
    key_down: bool,

    /// Position increment, used for internal processing.
    position_increment: f32,

    /// Voice priority.
    priority: u32,

    /// Audio file sample position, used for internal processing.
    sample_position: f32,

    /// Sample rate in Hz.
    sample_rate: f32,
}
impl AudioFileVoice {
    /// Creates new audio file voice.
    pub fn new() -> Self {
        AudioFileVoice {
            active_sound: None,
            adsr: LinearAdsr::new(0.001, 0.1),
            gain: 0.0,
            key_down: false,
            position_increment: 0.0,
            priority: 0,
            sample_position: 0.0,
            sample_rate: 44100.0,
        }
    }
}
impl SamplerVoice<AudioFileSound> for AudioFileVoice {
    fn get_active_note(&self) -> Option<u8> {
        if let Some((_, note)) = self.active_sound { Some(note) } else { None }
    }

    fn get_priority(&self) -> u32 {
        self.priority
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
                // Get sample.
                let envelope_gain = self.adsr.next_sample();
                let sample = sound.0.get_value(self.sample_position);

                // Mix sample into output buffer.
                frame[0] += sample.0 * envelope_gain * self.gain;
                frame[1] += sample.1 * envelope_gain * self.gain;

                // Advance sample position, possibly stop note if reached end of sample.
                self.sample_position += self.position_increment;
                if self.sample_position > sound.0.duration_samples as f32 { self.stop_note(0.0, false); break; }

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

    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: Arc<AudioFileSound>, initial_priority: u32) {
        self.adsr.set_parameters(sound.adsr.0, sound.adsr.3);
        self.adsr.note_on();
        self.gain = velocity / 4.0; // TODO
        self.position_increment =
            f32::powf(2.0, (midi_note as f32 - sound.midi_region.0 as f32) / 12.0)
            * (sound.sample_rate / self.sample_rate);
        self.sample_position = 0.0;
        self.active_sound = Some((sound, midi_note));
        self.priority = initial_priority;
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
    use std::path::PathBuf;

    #[test]
    fn play_note() {
        let mut buffer: Box<[f32]> = vec![0.0; 512].into_boxed_slice();
        let mut voice = AudioFileVoice::new();
        let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/test_sine.wav").to_str().unwrap().to_string();
        let sound = AudioFileSound::from_wav(&test_file, (48, 40, 50, 18, 120), (0.001, 0.0, 0.0, 0.1)).unwrap();
        voice.reset(1000.0, 512);

        voice.start_note(48, 1.0, Arc::new(sound), 0);
        assert_eq!(voice.get_active_note(), Some(48));

        voice.stop_note(0.0, true);
        assert_eq!(voice.is_playing(), true); // Allow tail.

        voice.render(&mut buffer);
        assert_eq!(voice.is_playing(), false);
    }

    #[test]
    fn render_sound() {
        let mut buffer: Box<[f32]> = vec![0.0; 2048].into_boxed_slice();
        let mut voice = AudioFileVoice::new();
        let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/test_sine.wav").to_str().unwrap().to_string();
        let sound = Arc::new(AudioFileSound::from_wav(&test_file, (48, 40, 60, 18, 120), (0.001, 0.0, 0.0, 0.1)).unwrap());
        voice.reset(sound.sample_rate * 2.0, buffer.len()); // Double sample rate -> 0.5x play rate.

        voice.start_note(60, 1.0, sound.clone(), 0); // One octave higher than root -> 2x play rate.
        voice.render(&mut buffer);

        for i in (256..buffer.len()).step_by(2) { // Skip adsr attack, stereo voice.
            let value = sound.get_value((i / 2) as f32);
            assert!((buffer[i+0] - value.0 / 4.0).abs() < 1e-16, "Unexpected (left) buffer value at index {}: got {} instead of {}", i+0, buffer[i+0], value.0);
            assert!((buffer[i+1] - value.1 / 4.0).abs() < 1e-16, "Unexpected (right) buffer value at index {}: got {} instead of {}", i+1, buffer[i+1], value.1);
        }
    }
}
