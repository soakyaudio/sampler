use crate::base::{AudioProcessor, ParameterId, ParameterValue, MidiMessage, MidiReceiver};
use std::{sync::{Arc, Mutex, RwLock}, collections::HashMap};

/// Encapsulates communication with an audio processor that lives on the audio thread.
pub struct ProcessorProxy {
    /// Parameter map populated by processor messages.
    parameter_map: Arc<RwLock<HashMap<ParameterId, ParameterValue>>>,

    /// Channel to send messages to the processor.
    to_source: Arc<Mutex<ringbuf::Producer<ProxyMessage>>>,
}
impl ProcessorProxy {
    /// Creates new processor proxy, returning message loop thread handle.
    fn new(to_source: ringbuf::Producer<ProxyMessage>, from_source: ringbuf::Consumer<ProcessorMessage>) -> (Self, std::thread::JoinHandle<()>) {
        let to_source = Arc::new(Mutex::new(to_source));
        let parameter_map = Arc::new(RwLock::new(HashMap::new()));
        let proxy = ProcessorProxy { parameter_map, to_source };
        let message_loop = proxy.start_message_loop(from_source);
        (proxy, message_loop)
    }

    /// Gets a parameter.
    pub fn get_parameter(&self, id: ParameterId) -> Option<ParameterValue> {
        let parameter_map = self.parameter_map.read().unwrap();
        if let Some(value) = parameter_map.get(&id) { Some(*value) } else { None }
    }

    /// Sends a parameter change to processor.
    pub fn set_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        self.to_source.lock().unwrap().push(ProxyMessage::SetParameter(id, value)).ok();
    }

    /// Starts thread that handles messages from processor.
    fn start_message_loop(&self, mut from_source: ringbuf::Consumer<ProcessorMessage>) -> std::thread::JoinHandle<()> {
        let parameter_map = self.parameter_map.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::park(); // wait for notification from audio thread
                let mut parameter_map = parameter_map.write().unwrap();
                while let Some(message) = from_source.pop() {
                    match message {
                        ProcessorMessage::UpdateParameter(id, value) => parameter_map.insert(id, value),
                    };
                }
            }
        })
    }
}
impl Clone for ProcessorProxy {
    fn clone(&self) -> Self {
        Self { parameter_map: self.parameter_map.clone(), to_source: self.to_source.clone() }
    }
}
impl MidiReceiver for ProcessorProxy {
    fn handle_midi_message(&mut self, message: MidiMessage) {
        self.to_source.lock().unwrap().push(ProxyMessage::HandleMidi(message)).ok();
    }
}

/// The processor end of the communication channel, serving the proxy.
pub struct ProcessorProxySource {
    /// Channel to receive messages from proxy.
    from_proxy: ringbuf::Consumer<ProxyMessage>,

    /// (First) proxy object with message loop thread handle.
    proxy: (ProcessorProxy, std::thread::JoinHandle<()>),

    /// Channel to send messages to proxy.
    to_proxy: ringbuf::Producer<ProcessorMessage>,
}
impl ProcessorProxySource {
    /// Creates new processor proxy source with specified message buffer size (upper limit for unprocessed messages).
    pub fn new(buffer_size: usize) -> Self {
        // Build channel from proxy to processor.
        let from_proxy_to_source = ringbuf::RingBuffer::<ProxyMessage>::new(buffer_size);
        let (to_source, from_proxy) = from_proxy_to_source.split();

        // Build channel from processor to proxy.
        let from_source_to_proxy = ringbuf::RingBuffer::<ProcessorMessage>::new(buffer_size);
        let (to_proxy, from_source) = from_source_to_proxy.split();

        // Create proxy.
        let proxy = ProcessorProxy::new(to_source, from_source);

        ProcessorProxySource { from_proxy, proxy, to_proxy }
    }

    /// Returns a proxy to this source.
    pub fn get_proxy(&self) -> ProcessorProxy {
        self.proxy.0.clone()
    }

    /// Handles proxy messages on a processor.
    pub fn handle_messages(&mut self, processor: &mut dyn AudioProcessor) {
        while let Some(message) = self.from_proxy.pop() {
            match message {
                ProxyMessage::HandleMidi(message) => processor.handle_midi_message(message),
                ProxyMessage::SetParameter(id, value) => processor.set_parameter(id, value),
            };
        }
    }

    /// Notifies proxy about new changes.
    pub fn notify_proxy(&self) {
        self.proxy.1.thread().unpark();
    }

    /// Sends a parameter update to proxy.
    pub fn update_parameter(&mut self, id: ParameterId, value: ParameterValue) {
        self.to_proxy.push(ProcessorMessage::UpdateParameter(id, value)).ok();
    }
}

/// Messages sent from proxy to processor.
#[derive(Debug)]
enum ProxyMessage {
    /// Handles a MIDI message.
    HandleMidi(MidiMessage),

    /// Sets a parameter.
    SetParameter(ParameterId, ParameterValue),
}

/// Messages sent from processor to proxy.
#[derive(Debug)]
enum ProcessorMessage {
    /// Updates a parameter.
    UpdateParameter(ParameterId, ParameterValue),
}
