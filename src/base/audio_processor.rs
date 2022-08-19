use crate::base::{ParameterId, ParameterValue};

/// Defines a generic audio processor, e.g. an instrument or effect.
pub trait AudioProcessor: Send {
    /// Gets a parameter.
    fn get_parameter(&self, id: &ParameterId) -> Option<ParameterValue>;

    /// Renders audio samples into a buffer.
    fn process(&mut self, buffer: &mut [f32]);

    /// Resets the processor's internal parameters.
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize);

    /// Sets a parameter.
    fn set_parameter(&mut self, id: &ParameterId, value: &ParameterValue);
}
