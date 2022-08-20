mod base;
mod engine;
mod processing;

use base::ParameterValue;
use processing::Sine;
use std::time::Duration;

fn main() {
    let sine = Sine::new(2);
    let (processor, mut proxy) = engine::CpalProcessor::new(Box::new(sine));
    let _engine = engine::CpalAudioEngine::new(processor);

    // Sine sweep.
    for i in 1..200 {
        println!("amplitude: {:?}, frequency: {:?}", proxy.get_parameter(Sine::Amplitude.id), proxy.get_parameter(Sine::Frequency.id));
        proxy.set_parameter(Sine::Frequency.id, ParameterValue::Float(i as f32 * 10.0));
        let ParameterValue::Float(amplitude) = proxy.get_parameter(Sine::Amplitude.id).unwrap();
        proxy.set_parameter(Sine::Amplitude.id, ParameterValue::Float(amplitude * 0.9));
        std::thread::sleep(Duration::from_millis(4));
    }
}
