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
impl SamplerSound for OscillatorSound {
    fn applies_to_note(&self, _midi_note: u8, _midi_velocity: u8) -> bool {
        true
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_sine() {
        let sound = OscillatorSound::new();
        assert!((sound.get_value(1.0) - f32::sin(1.0)).abs() < 1e-16);
        assert!((sound.get_value(2.0) - f32::sin(2.0)).abs() < 1e-16);
        assert!((sound.get_value(3.0) - f32::sin(3.0)).abs() < 1e-16);
    }
}
