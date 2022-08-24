/// Defines a sound that can be played by a sampler voice, e.g. an oscillator config or audio file data.
pub trait SamplerSound: Send + Sync {
    /// Returns whether sound applies to given midi note.
    fn applies_to_note(&self, midi_note: u8) -> bool;
}
