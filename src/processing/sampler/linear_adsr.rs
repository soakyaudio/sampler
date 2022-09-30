/// Linear ADSR envelope.
#[derive(Debug)]
pub struct LinearAdsr {
    /// Attack in seconds. Valid range is 0.001s to 10.0s.
    attack: f32,
    attack_delta: f32,

    /// Current envelope gain.
    envelope_gain: f32,

    /// Release in seconds. Valid range is 0.001s to 30.0s.
    release: f32,
    release_delta: f32,

    /// Sample rate in Hz.
    sample_rate: f32,

    /// Current stage if active, otherwise [None].
    stage: Option<AdsrStage>,
}
impl LinearAdsr {
    /// Create new linear ADSR.
    pub fn new(attack: f32, release: f32) -> Self {
        let mut adsr = LinearAdsr {
            attack: 0.0,
            attack_delta: 0.0,
            envelope_gain: 0.0,
            release: 0.0,
            release_delta: 0.0,
            sample_rate: 44100.0,
            stage: None,
        };
        adsr.set_parameters(attack, release);
        adsr
    }

    /// Returns whether ADSR is active.
    pub fn is_active(&self) -> bool {
        self.stage.is_some()
    }

    /// Returns next sample and advances ADSR state.
    pub fn next_sample(&mut self) -> f32 {
        if let Some(stage) = &self.stage {
            match stage {
                AdsrStage::Attack => {
                    self.envelope_gain = (self.envelope_gain + self.attack_delta).min(1.0);
                }
                AdsrStage::Release => {
                    self.envelope_gain -= self.release_delta;
                    if self.envelope_gain < 0.0 {
                        self.envelope_gain = 0.0;
                        self.stage = None;
                    }
                }
            };
        }
        self.envelope_gain
    }

    /// Note off, triggers envelope release.
    pub fn note_off(&mut self) {
        self.stage = Some(AdsrStage::Release);
    }

    /// Note on, triggers envelope attack.
    pub fn note_on(&mut self) {
        self.envelope_gain = 0.0;
        self.stage = Some(AdsrStage::Attack);
    }

    /// Set ADSR parameters.
    pub fn set_parameters(&mut self, attack: f32, release: f32) {
        // Clamp parameters.
        self.attack = attack.clamp(0.001, 10.0);
        self.release = release.clamp(0.001, 30.0);

        // Precalculate deltas.
        self.attack_delta = 1.0 / (self.attack * self.sample_rate);
        self.release_delta = 1.0 / (self.release * self.sample_rate);
    }

    /// Resets internal parameters of the envelope.
    pub fn reset(&mut self, sample_rate: f32) {
        self.envelope_gain = 0.0;
        self.sample_rate = sample_rate;
        self.stage = None;
        self.set_parameters(self.attack, self.release);
    }
}

/// ADSR stages.
#[derive(Debug)]
enum AdsrStage {
    Attack,
    Release,
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack() {
        let mut adsr = LinearAdsr::new(0.1, 0.5);
        let mut steps = 0;
        adsr.reset(1000.0);

        assert_eq!(adsr.next_sample(), 0.0);
        adsr.note_on();
        while adsr.next_sample() < 1.0 {
            steps += 1
        }
        assert_eq!(steps, 100);
    }

    #[test]
    fn release() {
        let mut adsr = LinearAdsr::new(0.001, 0.5);
        let mut steps = 0;
        adsr.reset(1000.0);
        adsr.note_on();
        adsr.next_sample();

        assert_eq!(adsr.next_sample(), 1.0);
        adsr.note_off();
        while adsr.next_sample() > 0.0 {
            steps += 1
        }
        assert_eq!(steps, 500);
        assert_eq!(adsr.is_active(), false);
    }
}
