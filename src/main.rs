mod base;
mod engine;
mod processing;

use processing::Sine;

fn main() {
    let sine = Sine::new(2);
    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sine));
    let _audio_engine = engine::CpalAudioEngine::new(processor);
    let _midi_engine = engine::MidirMidiEngine::new(proxy);
    std::thread::park();
}
