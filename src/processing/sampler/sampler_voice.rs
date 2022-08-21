use super::SamplerSound;

/// Defines a voice that a sampler can use to play a sampler sound.
pub trait SamplerVoice<'a, Sound: SamplerSound>: Send {
    /// Returns whether voice is currently in use.
    fn is_playing(&self) -> bool;

    /// Renders audio samples from sound into buffer (additive).
    fn render(&mut self, buffer: &mut [f32]);

    /// Resets internal parameters of the voice.
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize);

    /// Plays a note on this voice.
    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: &'a Sound);

    /// Stops note.
    fn stop_note(&mut self, velocity: f32, allow_tail: bool);
}
