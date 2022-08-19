use crate::base::{ParameterId, ParameterValue};
use std::{sync::{Arc, Mutex, RwLock}, collections::HashMap};

/// Encapsulates communication with an audio processor that lives on the audio thread.
pub struct ProcessorProxy {
    /// Parameter map populated by processor messages.
    parameter_map: Arc<RwLock<HashMap<ParameterId, ParameterValue>>>,

    /// Channel to send messages to the processor.
    to_source: Arc<Mutex<ringbuf::Producer<ProxyMessage>>>,
}
impl ProcessorProxy {
    /// Creates new [ProcessorProxy], returning message loop thread handle.
    fn new(to_source: ringbuf::Producer<ProxyMessage>, mut from_source: ringbuf::Consumer<ProcessorMessage>) -> (Self, std::thread::JoinHandle<()>) {
        let to_source = Arc::new(Mutex::new(to_source));
        let parameter_map = Arc::new(RwLock::new(HashMap::new()));
        let proxy = ProcessorProxy { parameter_map, to_source };
        let message_loop = proxy.start_message_loop(from_source);
        (proxy, message_loop)
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

/// The processor end of the communication channel, serving the proxies.
pub struct ProcessorProxySource {
    /// Channel to receive messages from proxies.
    from_proxy: ringbuf::Consumer<ProxyMessage>,

    /// (First) proxy object with message loop thread handle.
    proxy: (ProcessorProxy, std::thread::JoinHandle<()>),

    /// Channel to send messages to proxies.
    to_proxy: ringbuf::Producer<ProcessorMessage>,
}
impl ProcessorProxySource {
    /// Creates new [ProcessorProxySource] with specified message buffer size (upper limit for unprocessed messages).
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
}

/// Messages sent from proxy to processor.
#[derive(Debug)]
enum ProxyMessage {
    /// Sets a parameter.
    SetParameter(ParameterId, ParameterValue),
}

/// Messages sent from processor to proxy.
#[derive(Debug)]
enum ProcessorMessage {
    /// Updates a parameter.
    UpdateParameter(ParameterId, ParameterValue),
}
