use super::{SamplerVoice, AudioFileSound, LinearAdsr};
use std::{f32::consts::PI, sync::Arc};

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
            adsr: LinearAdsr::new(0.03, 0.1),
            gain: 0.0,
            key_down: false,
            position_increment: 0.0,
            sample_position: 0.0,
            sample_rate: 44100.0,
        }
    }
}
impl SamplerVoice<AudioFileSound> for AudioFileVoice {
    fn get_active_note(&self) -> Option<u8> {
        if let Some((_, note)) = self.active_sound { Some(note) } else { None }
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
                self.sample_position += self.position_increment;

                // Mix sample into output buffer.
                frame[0] += sample.0 * envelope_gain;
                frame[1] += sample.1 * envelope_gain;

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

    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: Arc<AudioFileSound>) {
        self.adsr.note_on();
        self.gain = velocity;
        self.position_increment =
            f32::powf(2.0, (midi_note as f32 - sound.midi_region().0 as f32) / 12.0)
            * (sound.sample_rate() / self.sample_rate);
        println!("{}", self.position_increment);
        self.sample_position = 0.0;
        self.active_sound = Some((sound, midi_note));
    }

    fn stop_note(&mut self, _velocity: f32, allow_tail: bool) {
        if allow_tail {
            self.adsr.note_off();
        } else {
            self.active_sound = None;
        }
    }
}
