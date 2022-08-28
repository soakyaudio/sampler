mod base;
mod engine;
mod processing;

use processing::{Sampler, AudioFileSound, AudioFileVoice};

fn main() {
    let mut sampler: Sampler<AudioFileSound, AudioFileVoice> = Sampler::new();

    for _ in 0..64 {
        sampler.add_voice(AudioFileVoice::new());
    }
    sampler.add_sound(AudioFileSound::from_wav("sample.wav", (48, 48, 48)).unwrap());

    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sampler));
    let _audio_engine = engine::CpalAudioEngine::new(processor);
    let _midi_engine = engine::MidirMidiEngine::new(proxy);

    std::thread::park();
}
