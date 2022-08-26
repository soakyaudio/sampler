use super::SamplerSound;

/// Audio file sound for sampler.
pub struct AudioFileSound {
    /// Channels in audio file / buffer.
    channel_count: u16,

    /// Audio file sample buffer.
    sample_buffer: Box<[f32]>,

    /// Audio file sample rate.
    sample_rate: f32,
}
impl AudioFileSound {
    /// Creates new audio file sound from WAV file.
    pub fn from_wav(file_path: &str) -> Result<Self, hound::Error> {
        // Read WAV samples into memory (disk streaming is planned for later).
        let mut reader = hound::WavReader::open(file_path)?;
        let format = reader.spec();
        let sample_buffer: Box<[f32]> = match format.sample_format {
            hound::SampleFormat::Float => {
                reader.samples().map(|s| s.unwrap()).collect()
            },
            hound::SampleFormat::Int => {
                let normalization_factor = f32::powi(2.0, format.bits_per_sample as i32 - 1);
                reader.samples::<i32>().map(|s| s.unwrap() as f32 / normalization_factor).collect()
            },
        };

        // Create sound object.
        let sound = AudioFileSound {
            channel_count: format.channels,
            sample_buffer,
            sample_rate: format.sample_rate as f32,
        };
        Ok(sound)
    }

    /// Returns duration in samples.
    pub fn duration_samples(&self) -> usize {
        self.sample_buffer.len() / self.channel_count as usize
    }

    /// Returns stereo sample value at position (supports linear interpolation).
    #[inline(always)]
    pub fn get_value(&self, sample_position: f32) -> (f32, f32) {
        let interleaved_index = sample_position as usize * self.channel_count as usize;
        match self.channel_count {
            1 => (self.sample_buffer[interleaved_index+0], self.sample_buffer[interleaved_index+0]),
            _ => (self.sample_buffer[interleaved_index+0], self.sample_buffer[interleaved_index+1]), // Ignore other channels.
        }
    }

    /// Returns file sample rate.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
impl SamplerSound for AudioFileSound {
    fn applies_to_note(&self, _midi_note: u8) -> bool {
        true
    }
}
