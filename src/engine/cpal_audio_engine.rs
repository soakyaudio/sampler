use crate::base::{AudioEngine, AudioProcessor};
use crate::engine::CpalProcessor;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

/// Audio engine based on [cpal].
pub struct CpalAudioEngine {
    /// Audio (output) stream.
    _stream: cpal::Stream,
}
impl CpalAudioEngine {
    /// Creates a new cpal audio engine, uses default audio config to init output stream.
    pub fn new(mut processor: CpalProcessor) -> Self {
        // Get default host, output device and config.
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device available.");
        let config = device
            .default_output_config()
            .expect("No default output stream config found.");

        // Sorry, only f32 for now.
        if config.sample_format() != cpal::SampleFormat::F32 {
            panic!("Default output stream config uses unsupported sample format.");
        }

        // Reset processor with config.
        let max_buffer_size = match config.buffer_size() {
            cpal::SupportedBufferSize::Range { min: _, max } => *max as usize,
            cpal::SupportedBufferSize::Unknown => 4096,
        };
        processor.reset(config.sample_rate().0 as f32, max_buffer_size);

        // Create output stream.
        let stream = {
            let audio_fn = move |buffer: &mut [f32], _: &cpal::OutputCallbackInfo| processor.process(buffer);
            let err_fn = move |err| eprintln!("An error occurred on the output audio stream: {}", err);
            device
                .build_output_stream(&config.into(), audio_fn, err_fn)
                .expect("Failed to create output stream.")
        };
        stream.play().expect("Failed to start output stream.");

        CpalAudioEngine { _stream: stream }
    }
}
impl AudioEngine for CpalAudioEngine {}
