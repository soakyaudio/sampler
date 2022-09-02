use crate::processing::{Sampler, AudioFileSound, AudioFileVoice};
use std::path::Path;

/// Loader for SFZ-based samplers.
pub struct SfzLoader {}
impl SfzLoader {
    /// Creates sampler from SFZ file.
    pub fn from_file(path: &str) -> Sampler<AudioFileSound, AudioFileVoice> {
        // Parse file and create sampler.
        let instrument = sofiza::Instrument::from_file(&Path::new(path))
            .expect("Failed to load SFZ file.");
        let sampler = Sampler::new();

        // Add voices based on polyphony opcode (defaults to 64).
        println!("{:?}", instrument);

        sampler
    }
}
