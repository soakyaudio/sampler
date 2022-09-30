mod base;
mod engine;
mod format;
mod processing;

#[cfg(test)]
mod test;

fn main() {
    let path = "samples/rhodes.sfz";
    let sampler = format::SfzLoader::from_file(path);
    let (processor, proxy) = engine::CpalProcessor::new(Box::new(sampler));
    let _audio_engine = engine::CpalAudioEngine::new(processor);
    let _midi_engine = engine::MidirMidiEngine::new(proxy);

    std::thread::park();
}
