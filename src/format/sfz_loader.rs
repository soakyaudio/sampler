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
        let mut sampler = Sampler::new();

        // Add voices based on polyphony opcode (defaults to 64).
        for _ in 0..64 {
            sampler.add_voice(AudioFileVoice::new());
        }

        // Add sounds.
        for region in instrument.regions {
            let mut sound_builder = AudioFileSoundBuilder::new();

            // Apply opcodes according to precedence.
            instrument.global.values().for_each(|opcode| sound_builder.apply(opcode));
            if let Some(group) = region.group {
                instrument.groups[group].opcodes.values().for_each(|opcode| sound_builder.apply(opcode));
            }
            region.opcodes.values().for_each(|opcode| sound_builder.apply(opcode));
            println!("{:?}", region.opcodes);

            // Add if valid.
            if let Ok(sound) = sound_builder.build() {
                sampler.add_sound(sound);
            }
        }

        println!("{:#?}", sampler);
        sampler
    }
}

/// Audio file sound builder.
struct AudioFileSoundBuilder {
    attack: f32,
    file_path: String,
    high_note: u8,
    high_velocity: u8,
    low_note: u8,
    low_velocity: u8,
    release: f32,
    root_note: u8,
}
impl AudioFileSoundBuilder {
    /// Creates new sound builder.
    fn new() -> AudioFileSoundBuilder {
        AudioFileSoundBuilder {
            attack: 0.001,
            file_path: String::from(""),
            high_note: 127,
            high_velocity: 127,
            low_note: 0,
            low_velocity: 0,
            release: 0.03,
            root_note: 48,
        }
    }

    /// Applies opcode to sound.
    fn apply(&mut self, opcode: &sofiza::Opcode) {
        match opcode {
            sofiza::Opcode::sample(path) => self.file_path = String::from(path.to_str().unwrap()),
            _ => (),
        }
    }

    /// Builds sound.
    fn build(&self) -> Result<AudioFileSound, ()> {
        if self.file_path.ends_with(".wav") {
            let midi_region = (self.root_note, self.low_note, self.high_note, self.low_velocity, self.high_velocity);
            let adsr = (self.attack, 0.0, 0.0, self.release);
            let sound = AudioFileSound::from_wav(&self.file_path, midi_region, adsr);
            if sound.is_err() { eprintln!("Failed to load sample: {}", self.file_path); }
            sound.or(Err(()))
        } else {
            eprintln!("Unsupported audio file format: {}", self.file_path);
            Err(())
        }
    }
}
