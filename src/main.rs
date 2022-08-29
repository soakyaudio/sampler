mod base;
mod engine;
mod processing;

#[cfg(test)]
mod test;

use processing::{Sampler, AudioFileSound, AudioFileVoice};

fn main() {
    let mut sampler: Sampler<AudioFileSound, AudioFileVoice> = Sampler::new();

    for _ in 0..64 {
        sampler.add_voice(AudioFileVoice::new());
    }
    sampler.add_sound(AudioFileSound::from_wav("sample.wav", (48, 0, 127, 0, 127), (0.001, 0.0, 0.0, 1.0)).unwrap());

    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sampler));
    let _audio_engine = engine::CpalAudioEngine::new(processor);
    let _midi_engine = engine::MidirMidiEngine::new(proxy);

    std::thread::park();
}
