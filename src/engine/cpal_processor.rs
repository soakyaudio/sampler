use crate::base::{AudioProcessor, Parameter, ParameterId, ParameterValue, ProcessorProxySource, ProcessorProxy};

/// Audio processor wrapper for the cpal audio engine, handling thread synchronization.
pub struct CpalProcessor {
    /// Wrapped audio processor.
    processor: Box<dyn AudioProcessor>,

    /// Proxy source, communicates with proxy objects.
    proxy_source: ProcessorProxySource,
}
impl CpalProcessor {
    /// Creates new cpal processor, including a proxy object to communicate with audio processor.
    pub fn new(processor: Box<dyn AudioProcessor>) -> (Self, ProcessorProxy) {
        // Create proxy source and proxy.
        let proxy_source = ProcessorProxySource::new(256);
        let proxy = proxy_source.get_proxy();
        let mut cpal_processor = CpalProcessor { processor, proxy_source };

        // Fetch initial data.
        cpal_processor.update_proxy();

        (cpal_processor, proxy)
    }

    // Updates proxy (e.g. parameters might have changed during processing).
    fn update_proxy(&mut self) {
        self.processor.list_parameters().iter().for_each(|p| {
            self.proxy_source.update_parameter(p.id, self.processor.get_parameter(p.id).unwrap());
        });
        self.proxy_source.notify_proxy();
    }
}
impl AudioProcessor for CpalProcessor {
    fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        self.processor.get_parameter(id)
    }
    fn list_parameters(&self) -> &[Parameter] {
        self.processor.list_parameters()
    }
    fn process(&mut self, buffer: &mut [f32]) {
        // Handle proxy messages.
        self.proxy_source.handle_messages(&mut *self.processor);

        // Delegate processing to wrapped processor.
        self.processor.process(buffer);

        // Update proxy.
        self.update_proxy();
    }
    fn reset(&mut self, sample_rate: f32, max_buffer_size: usize) {
        self.processor.reset(sample_rate, max_buffer_size);
    }
    fn set_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        self.processor.set_parameter(id, value);
    }
}
