mod base;
mod engine;
mod processing;

use processing::{Sampler, OscillatorSound, OscillatorVoice};

fn main() {
    let mut sampler: Sampler<OscillatorSound, OscillatorVoice> = Sampler::new();

    for _ in 0..64 {
        sampler.add_voice(OscillatorVoice::new());
    }
    sampler.add_sound(OscillatorSound::new());

    let _file = processing::AudioFileSound::from_wav("sample.wav").unwrap();
    println!("{}", _file.duration_samples());

    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sampler));
    let _audio_engine = engine::CpalAudioEngine::new(processor);
    let _midi_engine = engine::MidirMidiEngine::new(proxy);

    // std::thread::park();
}
