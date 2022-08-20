mod base;
mod engine;
mod processing;

fn main() {
    let sine = processing::Sine::new(2);
    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sine));
    let _engine = engine::CpalAudioEngine::new(processor);
    std::thread::park();
}
