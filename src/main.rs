mod base;
mod engine;
mod processing;

use processing::{Sampler, OscillatorSound, OscillatorVoice};

fn main() {
    // let sampler = Sampler::<OscillatorSound, OscillatorVoice<OscillatorSound>>::new();
    // let (processor, proxy) = engine::CpalProcessor::new(Box::new(sampler));
    // let _audio_engine = engine::CpalAudioEngine::new(processor);
    // let _midi_engine = engine::MidirMidiEngine::new(proxy);

    std::thread::park();
}
