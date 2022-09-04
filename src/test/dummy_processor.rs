use crate::base::{AudioProcessor, ParameterId, ParameterValue, Parameter, MidiReceiver, MidiMessage};
use std::collections::HashMap;

/// Dummy audio processor for testing purposes.
pub struct DummyProcessor {
    pub midi_messages: Vec<MidiMessage>,
    parameter: HashMap<ParameterId, ParameterValue>,
}
impl DummyProcessor {
    pub fn new() -> DummyProcessor {
        let mut processor = DummyProcessor {
            midi_messages: Vec::new(),
            parameter: HashMap::new(),
        };
        processor.set_parameter(0, ParameterValue::Float(0.0));
        processor.set_parameter(1, ParameterValue::Float(0.0));
        processor
    }
}
impl AudioProcessor for DummyProcessor {
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        Some(*self.parameter.get(&id).unwrap())
    }
    fn list_parameters(&self) -> &[Parameter] {
        const PARAMS: [Parameter; 2] = [Parameter::new(0, "param0"), Parameter::new(1, "param1")];
        &PARAMS
    }
    fn process(&mut self, _buffer: &mut [f32]) {
        return;
    }
    fn reset(&mut self, _sample_rate: f32, _max_buffer_size: usize) {
        return;
    }
    fn set_channel_layout(&mut self, _input_channels: u16, _output_channels: u16) {
        return;
    }
    fn set_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        self.parameter.insert(id, value);
    }
}
impl MidiReceiver for DummyProcessor {
    fn handle_midi_message(&mut self, message: MidiMessage) {
        self.midi_messages.push(message);
    }
}
