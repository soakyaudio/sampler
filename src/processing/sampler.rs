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
    /// Number of output channels.
    channel_count: u16,

    /// Internal audio buffer (to mix stereo to mono).
    internal_buffer: Box<[f32]>,

    /// Sampler sounds.
    sounds: Vec<Arc<Sound>>,

    /// Sampler voices.
    voices: Vec<Voice>,
}
impl<S: SamplerSound, V: SamplerVoice<S>> Sampler<S, V> {
    /// Creates new sampler.
    pub fn new() -> Self {
        Sampler {
            channel_count: 0,
            internal_buffer: Box::new([]),
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

    /// Note off (usually triggered by a MIDI message).
    fn note_off(&mut self, midi_channel: u8, midi_note: u8, velocity: u8) {
        self.voices.iter_mut()
            .filter(|voice| voice.get_active_note() == Some(midi_note))
            .for_each(|voice| voice.stop_note(velocity as f32 / 127.0, true));
    }

    /// Note on (usually triggered by a MIDI message).
    fn note_on(&mut self, midi_channel: u8, midi_note: u8, velocity: u8) {
        // Find free voice.
        let voice = self.voices.iter_mut().find(|voice| !voice.is_playing());
        if let Some(voice) = voice {
            let sound = self.sounds.first().unwrap().clone();
            voice.start_note(midi_note, velocity as f32 / 127.0, sound);
        }
    }
}
impl<S: SamplerSound, V: SamplerVoice<S>> AudioProcessor for Sampler<S, V> {
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        todo!()
    }

    fn list_parameters(&self) -> &[Parameter] {
        &[]
    }

    fn process(&mut self, out_buffer: &mut [f32]) {
        // Prepare internal buffer.
        let frame_count = out_buffer.len() / self.channel_count as usize;
        let internal_buffer = &mut self.internal_buffer[0..2*frame_count]; // Stereo.
        internal_buffer.fill(0.0);

        // Render voices.
        self.voices.iter_mut().for_each(|voice| voice.render(internal_buffer));

        // Mix internal buffer into output buffer.
        if self.channel_count == 1 {
            for (out, int) in out_buffer.iter_mut().zip(internal_buffer.chunks_mut(2)) {
                *out = 0.5 * (int[0] + int[1]); // Mono output.
            }
        } else {
            for (out, int) in out_buffer.chunks_mut(self.channel_count as usize).zip(internal_buffer.chunks_mut(2)) {
                (out[0], out[1]) = (int[0], int[1]); // Stereo output, ignore other channels.
            }
        }
    }

    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize) {
        // Allocate internal resources.
        self.internal_buffer = vec![0.0; 2 * max_buffer_size].into_boxed_slice();

        // Reset voices.
        self.voices.iter_mut().for_each(|voice| voice.reset(sample_rate, max_buffer_size));
    }

    fn set_channel_layout(&mut self, _input_channels: u16, output_channels: u16) {
        self.channel_count = output_channels;
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
