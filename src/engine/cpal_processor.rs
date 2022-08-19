use crate::base::{AudioProcessor, ParameterId, ParameterValue};

/// [AudioProcessor] wrapper for the cpal audio engine, handling thread synchronization.
pub struct CpalProcessor {
    /// Wrapped [AudioProcessor].
    processor: Box<dyn AudioProcessor>,
}
impl CpalProcessor {
    /// Creates new [CpalProcessor].
    pub fn new(processor: Box<dyn AudioProcessor>) -> Self {
        CpalProcessor { processor }
    }
}
impl AudioProcessor for CpalProcessor {
    fn get_parameter(&self, id: &ParameterId) -> Option<ParameterValue> {
        self.processor.get_parameter(id)
    }
    fn process(&mut self, buffer: &mut [f32]) {
        self.processor.process(buffer);
    }
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize) {
        self.processor.reset(sample_rate, max_buffer_size);
    }
    fn set_parameter(&mut self, id: &ParameterId, value: &ParameterValue) {
        self.processor.set_parameter(id, value);
    }
}
