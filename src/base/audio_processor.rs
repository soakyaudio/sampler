use crate::base::{Parameter, ParameterId, ParameterValue, MidiReceiver};

/// Defines a generic audio processor, e.g. an instrument or effect.
pub trait AudioProcessor: Send + MidiReceiver {
    /// Gets a parameter.
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue>;

    /// Returns a list of available parameters.
    fn list_parameters(&self) -> &[Parameter];

    /// Renders audio samples into a buffer.
    fn process(&mut self, buffer: &mut [f32]);

    /// Resets the processor's internal parameters.
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize);

    /// Sets channel layout.
    fn set_channel_layout(&mut self, input_channels: u16, output_channels: u16);

    /// Sets a parameter.
    fn set_parameter(&mut self, id: ParameterId, value: ParameterValue);
}
