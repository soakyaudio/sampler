use super::SamplerSound;

/// Oscillator sound for sampler.
pub struct OscillatorSound {}
impl OscillatorSound {
    /// Creates new oscillator sound.
    pub fn new() -> Self {
        OscillatorSound {}
    }

    /// Returns sample value depending on oscillator mode.
    #[inline(always)]
    pub fn get_value(&self, phase: f32) -> f32 {
        f32::sin(phase) // Only sine waves for now.
    }
}
impl SamplerSound for OscillatorSound {}
