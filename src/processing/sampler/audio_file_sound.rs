use super::SamplerSound;

/// Audio file sound for sampler.
pub struct AudioFileSound {
    /// ADSR envelope in seconds.
    pub adsr: (f32, f32, f32, f32),

    /// Channels in audio file / buffer.
    channel_count: u16,

    /// Duration in samples.
    pub duration_samples: usize,

    /// Root midi note, lowest midi note, highest midi note.
    pub midi_region: (u8, u8, u8),

    /// Audio file sample buffer.
    sample_buffer: Box<[f32]>,

    /// Audio file sample rate.
    pub sample_rate: f32,
}
impl AudioFileSound {
    /// Creates new audio file sound from WAV file.
    pub fn from_wav(file_path: &str, midi_region: (u8, u8, u8), adsr: (f32, f32, f32, f32)) -> Result<Self, hound::Error> {
        // Read WAV samples into memory (disk streaming is planned for later).
        let mut reader = hound::WavReader::open(file_path)?;
        let format = reader.spec();
        let sample_buffer: Box<[f32]> = match format.sample_format {
            hound::SampleFormat::Float => {
                reader.samples::<f32>().map(|s| s.unwrap()).collect()
            },
            hound::SampleFormat::Int => {
                let normalization_factor = f32::powi(2.0, format.bits_per_sample as i32 - 1);
                reader.samples::<i32>().map(|s| s.unwrap() as f32 / normalization_factor).collect()
            },
        };

        // Add padding for linear interpolation.
        let duration_samples = sample_buffer.len() / format.channels as usize;
        let padding: Box<[f32]> = vec![0.0; format.channels as usize].into_boxed_slice();
        let sample_buffer = [sample_buffer, padding].concat().into_boxed_slice();

        // Create sound object.
        let sound = AudioFileSound {
            adsr,
            channel_count: format.channels,
            duration_samples,
            midi_region,
            sample_buffer,
            sample_rate: format.sample_rate as f32,
        };
        Ok(sound)
    }

    /// Returns stereo sample value at position (via linear interpolation).
    #[inline(always)]
    pub fn get_value(&self, sample_position: f32) -> (f32, f32) {
        // Interpolation example: sample[2.25] = (0.75 * sample[2]) + (0.25 * sample[3]).
        let index = sample_position as usize;
        let alpha = sample_position - index as f32;
        let inv_alpha = 1.0 - alpha;

        // Samples are stored interleaved.
        let interleaved_index_0 = (index+0) * self.channel_count as usize;
        let interleaved_index_1 = (index+1) * self.channel_count as usize;

        // Mirror left channel if mono, ignore channels beyond stereo.
        let l = inv_alpha * self.sample_buffer[interleaved_index_0] + alpha * self.sample_buffer[interleaved_index_1];
        let r = match self.channel_count {
            1 => l,
            _ => inv_alpha * self.sample_buffer[interleaved_index_0+1] + alpha * self.sample_buffer[interleaved_index_1+1],
        };
        (l, r)
    }
}
impl SamplerSound for AudioFileSound {
    fn applies_to_note(&self, midi_note: u8) -> bool {
        midi_note >= self.midi_region.1 && midi_note <= self.midi_region.2
    }
}
