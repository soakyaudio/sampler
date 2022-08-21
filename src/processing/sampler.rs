mod oscillator_sound;
mod oscillator_voice;
mod sampler_sound;
mod sampler_voice;

use std::{cell::RefCell, sync::Arc};

use crate::base::{MidiMessage, AudioProcessor, Parameter, ParameterId, ParameterValue, MidiReceiver};
pub use oscillator_sound::OscillatorSound;
pub use oscillator_voice::OscillatorVoice;
pub use sampler_sound::SamplerSound;
pub use sampler_voice::SamplerVoice;

/// Sampler instrument processor.
pub struct Sampler<Sound, Voice>
where
    Sound: SamplerSound,
    Voice: SamplerVoice<Sound>,
{
    /// Sampler sounds.
    sounds: Vec<Arc<Sound>>,

    /// Sampler voices.
    voices: Vec<Voice>,
}
impl<S: SamplerSound, V: SamplerVoice<S>> Sampler<S, V> {
    /// Creates new sampler.
    pub fn new() -> Self {
        Sampler {
            sounds: Vec::new(),
            voices: Vec::new(),
        }
    }

    /// Adds a sound.
    pub fn add_sound(&mut self, sound: S) {
        self.sounds.push(Arc::new(sound));
    }

    /// Adds a voice.
    pub fn add_voice(&mut self, voice: V) {
        self.voices.push(voice);
    }

    /// Note off (usually triggered by a midi message).
    fn note_off(&mut self, midi_channel: u8, midi_note: u8, velocity: u8) {

    }

    /// Note on (usually triggered by a midi message).
    fn note_on(&mut self, midi_channel: u8, midi_note: u8, velocity: u8) {
        let voice = self.voices.first_mut().unwrap();
        let sound = self.sounds.first().unwrap();
        voice.start_note(midi_note, velocity as f32 / 127.0, sound.clone());
    }
}
impl<S: SamplerSound, V: SamplerVoice<S>> AudioProcessor for Sampler<S, V> {
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        todo!()
    }

    fn list_parameters(&self) -> &[Parameter] {
        todo!()
    }

    fn process(&mut self, buffer: &mut [f32]) {
        // Render voices.
        self.voices.iter_mut().for_each(|voice| voice.render(buffer));
    }

    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize) {
        // Reset voices.
        self.voices.iter_mut().for_each(|voice| voice.reset(sample_rate, max_buffer_size));
    }

    fn set_channel_layout(&mut self, input_channels: u16, output_channels: u16) {
        todo!()
    }

    fn set_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        todo!()
    }
}
impl<S: SamplerSound, V: SamplerVoice<S>> MidiReceiver for Sampler<S, V> {
    fn handle_midi_message(&mut self, message: MidiMessage) {
        match message {
            MidiMessage::NoteOff(channel, note, velocity) => self.note_off(channel, note, velocity),
            MidiMessage::NoteOn(channel, note, velocity) => self.note_on(channel, note, velocity),
        }
    }
}
