use crate::processing::{SamplerVoice, SamplerSound};

/// Dummy sampler voice for testing purposes.
pub struct DummyVoice {
    active_note: Option<u8>,
    key_down: bool,
}
impl DummyVoice {
    pub fn new() -> Self {
        DummyVoice {
            active_note: None,
            key_down: false,
        }
    }
}
impl SamplerVoice<DummySound> for DummyVoice {
    fn get_active_note(&self) -> Option<u8> {
        self.active_note
    }
    fn get_priority(&self) -> u32 {
        0
    }
    fn is_key_down(&self) -> bool {
        self.key_down
    }
    fn is_playing(&self) -> bool {
        self.active_note.is_some()
    }
    fn render(&mut self, buffer: &mut [f32]) {
        if let Some(note) = self.active_note {
            buffer.chunks_mut(2).for_each(|frame| { // Stereo.
                frame[0] += note as f32 / 127.0;
                frame[1] += 0.1;
            });
        }
    }
    fn reset(&mut self, _sample_rate: f32, _max_buffer_size: usize) {

    }
    fn set_key_down(&mut self, key_down: bool) {
        self.key_down = key_down
    }
    fn start_note(&mut self, midi_note: u8, _velocity: f32, _sound: std::sync::Arc<DummySound>, _initial_priority: u32) {
        self.active_note = Some(midi_note);
    }
    fn stop_note(&mut self, _velocity: f32, _allow_tail: bool) {
        self.active_note = None;
    }
}

/// Dummy sampler sound for testing purposes.
pub struct DummySound {}
impl DummySound {
    pub fn new() -> Self {
        DummySound {}
    }
}
impl SamplerSound for DummySound {
    fn applies_to_note(&self, _midi_note: u8, _midi_velocity: u8) -> bool {
        true
    }
}
