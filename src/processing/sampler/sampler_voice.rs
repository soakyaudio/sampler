use std::sync::Arc;

use super::SamplerSound;

/// Defines a voice that a sampler can use to play a sampler sound.
pub trait SamplerVoice<Sound: SamplerSound>: Send {
    /// Returns current MIDI note if playing, [None] otherwise.
    fn get_active_note(&self) -> Option<u8>;

    // Returns voice priority (used for voice stealing, voices with lower priority are stolen first).
    fn get_priority(&self) -> u32;

    /// Returns key down state.
    fn is_key_down(&self) -> bool;

    /// Returns whether voice is currently in use.
    fn is_playing(&self) -> bool;

    /// Renders audio samples from sound into buffer (additive).
    fn render(&mut self, buffer: &mut [f32]);

    /// Resets internal parameters of the voice.
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize);

    /// Sets key down state.
    fn set_key_down(&mut self, key_down: bool);

    /// Plays a note on this voice.
    fn start_note(&mut self, midi_note: u8, velocity: f32, sound: Arc<Sound>, initial_priority: u32);

    /// Stops note.
    fn stop_note(&mut self, velocity: f32, allow_tail: bool);
}
